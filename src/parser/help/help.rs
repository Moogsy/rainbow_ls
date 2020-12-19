pub const TXT: &str = r#"
##########################
# Per entry type control #
##########################

--files [codes] (default=1)
--directories [codes] (default=17)
--symlinks [codes] (default=13)
--unknowns [codes] (default=14)

Where codes is one or more of:
    0 - Normal Style
    1 - Bold
    2 - Dim
    3 - Italic
    4 - Underlined
    5 - Blinking
    7 - Reverse
    8 - Invisible
 
Specify some formatting code to use for an entry type.

Examples:
    --files 1   // file is bold
    --files 12  // file is bold and dim
    --files 1 --dir 2 // file is bold, dir is dim

#################
# Color control #
#################

--sum [lowest_sum] (default=512)
Specifies the minimal sum of the red, green and blue
components of the colors. Cannot be over 765 (255 * 3).

Examples:
    --sum 512 // This will be bright
    --sum 100 // This will have a wide range, from very dark to very bright

##############
# Separators #
##############

--separator [separator_string] (default="  ")
Specifies which separator to use between filenames

--padding [padding_string] (default=" ")
Specifies which padding char will be used to align filenames in columns

Examples:
    --separator "-" --padding "+" // will show files as: file1+-file2
    --separator "~~" --padding " " // will show files as: file1 ~~file2

#################
# Miscellaneous #
#################

--read-graphenes [true/false] (default=true)
Specifies whether to read filenames per graphenes or not.
Let it turned on if you use non-english characters to keep alignment.
Please note that it turns filename length's determination into an O(n) operation.

--show-dotfiles [true/false] (default=false)
Specifies whether to show files that have names starting with a "." or not.
"#;