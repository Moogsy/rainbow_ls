pub const TXT: &str = r#"

######################################################
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
######################################################

##########
# Kwargs #
##########

--sum [minimal_sum] (default=512)
Specifies the minimal sum of the red, green and blue
components of the colors. Cannot be over 765 (255 * 3).

--separator [separator_string] (default="  ")
Specifies which separator to use between filenames

--padding [padding_string] (default=" ")
Specifies which padding char will be used to align filenames inside columns

--sort [word] (default=name)
Sort by word instead of name.
Where word is one of:
    size
    extension
    creation_date
    access_date
    modification_date

#########
# Flags #
#########

-a | --show-dotfiles | --all [true/false] (default=false)
Show all files, including the ones with names starting with a ".".
Note: does not display implied "." and ".." folders.

-r | --reverse (default=false)
Reverse the order of files when sorting.





"#;