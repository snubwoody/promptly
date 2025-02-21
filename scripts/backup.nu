
# The names of these colors are the same as the ones
# from the catpuccin palette to ensure consistency and 
# may not exactly match their usage in the theme

let mauve = "#8839ef"
let pink = "#ea76cb"
let rose_water = "#dc8a78"
let flamingo = "#dd7878"
let lavendar = "#7287fd"
let text = "#4c4f69"
let subtext1 = "#5c5f77"
let subtext0 = "#6c6f85"
let crust = "#dce0e8"

# Color spec
# Executable files:  
# Config files:
# Maybe use the official color  
# Source code files:  
# Dll and similar files:  
# Hidden directories:  

# Not all colors work please check
let catpuccin_latte_dark = {
	# Table config
	header: $crust #Table headers
	row_index: $flamingo #Row index in tables
	separator: $crust #Table border (might also affect other things)

	date: $pink
	cursor: $pink
	
	leading_trailing_space_bg: $pink
	env: $crust
	prompt: $crust
	dir: $pink
	path: $pink
	filesize: $subtext0
	duration: $crust
	string: $crust
	int: $pink
	bool: $pink
	block: $pink
	glob: $mauve
	hints: $text #Prompt hints
	exe: $pink
	
	# Do not know what these do
	shape_string: $pink
	shape_filepath: $pink
	shape_custom: $pink
	shape_keyword: $mauve
	shape_range: $pink
	shape_table: $pink
	shape_variable: $pink
}

let config_explore = {
	status_bar_background: $pink
	command_bar_text: $pink
	highlight: $pink
	selected_cell: $pink
}

$env.config = {
	show_banner: false
	color_config: $catpuccin_latte_dark
	explore: $config_explore
}

def time [] {
	date now | format date "%H:%M:%S"
}

def path [] {
	# Get only the two most down paths
	$env.PWD | path split | last 2 | path join
}

$env.PROMPT_COMMAND = {||$"~\\..\\(path)\n>" }
$env.PROMPT_COMMAND_RIGHT = {|| time}

# Prevent duplicate indicator
$env.PROMPT_INDICATOR = ""

$env.LS_COLORS = "di=0;34:*.exe=1;1;1"