#!/usr/bin/env nu

# Run fresh base and PR benchmarks serially on one GitHub Actions runner.
# Invoke from the root checkout:
#   nu .github/scripts/benchmark-pair.nu revisions
#   nu .github/scripts/benchmark-pair.nu run
#   nu .github/scripts/benchmark-pair.nu sizes
#
# `revisions` reads GITHUB_EVENT_NAME and GITHUB_EVENT_PATH. Manual dispatch
# also uses GITHUB_REPOSITORY, PR_NUMBER, and GH_TOKEN to fetch the PR with gh.
# It writes checkout outputs to GITHUB_OUTPUT and saves revisions.json plus
# pr-info/PR_NUMBER under BENCHMARK_RESULTS for execution and comment posting.
#
# `run` expects those revisions and a PR checkout at
# GITHUB_WORKSPACE/benchmark-workspace at the PR's merge revision. Fetch the
# base commit with checkout's configured Git credentials, then benchmark both
# revisions in this one checkout. GITHUB_SERVER_URL supplies the Git host.
# Switch revisions after the base build so Git updates changed files and
# Cargo reuses unchanged artifacts in the checkout's normal target/ directory.
# Keep that directory intact when switching revisions.
# Start and finish on the merge revision for rust-cache's dependency key and cleanup.
# The workflow selects Rust and supplies absolute BENCHMARK_RESULTS and
# CRITERION_HOME paths. Measurements are fresh and stay outside the build cache.
# `sizes` uses gh to fetch the Linux x64 size reports produced by CI for the
# same two commits. Run the report script afterward to compare the results.

# Resolve exact commit SHAs and their repositories for automatic or manual runs.
# Write base_sha, base_repository, and pr_sha to GITHUB_OUTPUT.
# Save revisions.json and pr-info/PR_NUMBER beneath BENCHMARK_RESULTS.
def 'main revisions' [] {
    let event = open $env.GITHUB_EVENT_PATH
    let manual = $env.GITHUB_EVENT_NAME == workflow_dispatch
    let pr = if $manual {
        ^gh api $"repos/($env.GITHUB_REPOSITORY)/pulls/($env.PR_NUMBER)" | from json
    } else { $event.pull_request }
    let pr_sha = if $manual { $pr.merge_commit_sha } else { $env.GITHUB_SHA }
    if $pr_sha == null {
        error make {msg: "GitHub has no merge revision for this PR."}
    }
    let base_sha = if $manual {
        ^gh api $"repos/($pr.base.repo.full_name)/git/commits/($pr_sha)" --jq '.parents[0].sha' | str trim
    } else { $pr.base.sha }
    let revisions = {
        base_sha: $base_sha, base_repository: $pr.base.repo.full_name,
        pr_sha: $pr_sha
    }
    mkdir ($env.BENCHMARK_RESULTS | path join pr-info)
    $pr.number | into string | save --raw -f ($env.BENCHMARK_RESULTS | path join pr-info/PR_NUMBER)
    $revisions | save -f ($env.BENCHMARK_RESULTS | path join revisions.json)
    $revisions | transpose key value | each {|item| $"($item.key)=($item.value)\n" }
        | str join | save --append $env.GITHUB_OUTPUT
}

# Buffer a command's stdout/stderr, save both verbatim, print them, and return
# its exit code. A process failure remains data so the other arm can still run.
def capture [
    command: closure # The external command to run.
    output: path # Stdout file; stderr is saved alongside it with a .stderr suffix.
] {
    let result = do $command | complete
    $result.stdout | save --raw -f $output
    $result.stderr | save --raw -f $"($output).stderr"
    print -n $result.stdout
    print -e -n $result.stderr
    $result.exit_code
}

# Run base and PR benchmarks with the workflow-selected compiler.
# Save compiler/CPU information and per-arm logs. Attempt both arms before
# failing with the collected command statuses and Criterion errors.
# Skip an arm if its revision cannot be checked out, but attempt the other arm.
def 'main run' [] {
    let revisions = open ($env.BENCHMARK_RESULTS | path join revisions.json)
    ^rustc -vV | save --raw -f ($env.BENCHMARK_RESULTS | path join environment.txt)
    ^lscpu | save --raw --append ($env.BENCHMARK_RESULTS | path join environment.txt)
    print (open --raw ($env.BENCHMARK_RESULTS | path join environment.txt))
    mkdir $env.CRITERION_HOME
    cd ($env.GITHUB_WORKSPACE | path join benchmark-workspace)
    mut failures = []
    for arm in [{name: base, sha: $revisions.base_sha} {name: pr, sha: $revisions.pr_sha}] {
        let results = $env.BENCHMARK_RESULTS | path join $arm.name
        mkdir $results
        print $"Benchmarking ($arm.name) at ($arm.sha)"
        if $arm.name == base {
            let fetch = $results | path join fetch.txt
            let repository = $"($env.GITHUB_SERVER_URL)/($revisions.base_repository).git"
            let status = capture { ^git fetch --no-tags --depth 1 $repository $arm.sha } $fetch
            if $status != 0 {
                $failures = $failures | append $"($arm.name) at ($arm.sha): git fetch exited ($status)
Logs: ($fetch), ($fetch).stderr"
                continue
            }
        }
        let checkout = $results | path join checkout.txt
        let status = capture { ^git checkout --detach $arm.sha } $checkout
        if $status != 0 {
            $failures = $failures | append $"($arm.name) at ($arm.sha): git checkout exited ($status)
Logs: ($checkout), ($checkout).stderr"
            continue
        }
        let log = $results | path join output.txt
        let status = capture {
            # Exclude the synthetic `wrap_stress` scaling target from automated runs.
            ^cargo bench --locked -p typstyle-core --bench pretty_print --bench typst_docs -- --save-baseline $arm.name
        } $log
        # Criterion can report failed cases on stdout while returning success.
        let errors = open --raw $log | lines | where {|line| $line | str contains 'Criterion.rs ERROR:' }
        if $status != 0 or not ($errors | is-empty) {
            $failures = $failures | append $"($arm.name) at ($arm.sha): cargo bench exited ($status)
($errors | str join (char newline))
Logs: ($log), ($log).stderr"
        }
    }
    if not ($failures | is-empty) {
        error make {msg: $"Benchmark failures:\n($failures | str join (char newline))"}
    }
}

# Allow the PR's CI build a short time to finish if Criterion finishes first.
# The base report may predate size reporting or have expired.
def 'main sizes' [] {
    let revisions = open ($env.BENCHMARK_RESULTS | path join revisions.json)
    for arm in [
        {name: base, sha: $revisions.base_sha, event: push, attempts: 1}
        {name: pr, sha: $revisions.pr_sha, event: pull_request, attempts: 12}
    ] {
        let destination = $env.BENCHMARK_RESULTS | path join $arm.name
        try {
            let found = download_size_report $revisions.base_repository $arm.sha $arm.event $arm.attempts $destination
            if not $found {
                print -e $"No release size report for ($arm.name) at ($arm.sha)"
            }
        } catch {|failure|
            print -e $"Release size report unavailable for ($arm.name) at ($arm.sha): ($failure.msg)"
        }
    }
}

# Find an artifact named for the exact checkout SHA and verify its CI run
# before downloading. The PR workflow only reads the report as data.
def download_size_report [repo: string, sha: string, event: string, attempts: int, destination: path] {
    let name = $"typstyle-linux-x64-size-($sha)"
    for attempt in 1..$attempts {
        let artifacts = ^gh api --method GET $"repos/($repo)/actions/artifacts" -f $"name=($name)" | from json | get artifacts
        for artifact in ($artifacts | where expired == false) {
            let run_id = $artifact.workflow_run.id
            let run = ^gh api $"repos/($repo)/actions/runs/($run_id)" | from json
            if $run.path == '.github/workflows/ci.yml' and $run.event == $event {
                ^gh run download $run_id --repo $repo --name $name --dir $destination
                return true
            }
        }
        if $attempt < $attempts {
            sleep 5sec
        }
    }
    false
}

# Resolve PR revisions, then run both benchmark arms; see the subcommand help.
def main [] { help main }
