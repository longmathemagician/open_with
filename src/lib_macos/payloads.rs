pub const GET_INFO: &str =
    "tell application \"Finder\" to open information window of file file_path";

pub const OPEN_WITH: &str =
"tell application \"Finder\" to set selection to file_path
tell application \"System Events\" to tell process \"Finder\"
	set application_list_raw to name of menu items of menu 1 of menu item \"Open With\" of menu 3 of menu bar 1
end tell
set text item delimiters to linefeed
set application_list to do shell script \"grep -vx \'missing value\' <<< \" & quoted form of (application_list_raw as text) & \" | grep -vx Other… | sed -E 's/ \\\\([^)]*\\\\)$//g;s/ \\\\(default\\\\)$//g;s/\\\\.app$//g' | sort -f | uniq\"
set application_selection_result to choose from list (paragraphs of application_list) without multiple selections allowed
if application_selection_result is false then return
set selected_application to item 1 of application_selection_result
set selected_application_path to (path to application selected_application)
tell application \"Finder\" to open selection using selected_application_path";

