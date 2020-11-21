pub const TXT: &str = r#"
##########################
# Per entry type control #
##########################

--files [codes] 
--directories [codes] 
--symlink [codes] 
--unknown [codes]  

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
    --file 1   // file is bold
    --file 12  // file is bold and dim
    --file 1 --dir 2 // file is bold, dir is dim

#################
# Color control #
#################

--sum [lowest_sum]
Specifies the minimal sum of the red, green and blue
components of the colors. Cannot be over 765 (255 * 3).
Example:
    --sum 512 // This will be bright
    --sum 100 // This will have a wide range, from very dark to very bright
"#;