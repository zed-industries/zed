## Triage Watcher v0.1
# This is a small script to watch for new issues on the Zed repository and open them in a new browser tab interactively.
#
## Installing Julia
#
# You need Julia installed on your system:
# curl -fsSL https://install.julialang.org | sh
#
## Running this script:
# 1. It only works on Macos/Linux
# Open a new Julia repl with `julia` inside the `zed` repo
# 2. Paste the following code
# 3. Whenever you close your computer, just type the Up arrow on the REPL + enter to rerun the loop again to resume
function get_issues()
    entries = filter(x -> occursin("state:needs triage", x), split(read(`gh issue list -L 10`, String), '\n'))
    top = findfirst.('\t', entries) .- 1
    [entries[i][begin:top[i]] for i in eachindex(entries)]
end

nums = get_issues();
while true
    new_nums = get_issues()
    # Open each new issue in a new browser tab
    for issue_num in setdiff(new_nums, nums)
        url = "https://github.com/zed-industries/zed/issues/" * issue_num
        println("\nOpening $url")
        open_tab = `/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome $url`
        try
            sound_file = "/Users/mrg/Downloads/mario_coin_sound.mp3"
            run(`afplay -v 0.02 $sound_file`)
        finally
        end
        run(open_tab)
    end
    nums = new_nums
    print("🧘🏼")
    sleep(60)
end
