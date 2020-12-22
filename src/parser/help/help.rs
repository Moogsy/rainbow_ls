pub const TXT: &str = r#"
##########
# Kwargs #
##########

--files [codes] (default=None)
--directories [codes] (default=4)
--symlinks [codes] (default=1)
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

--files-prefix [prefix] (default=None)
--dotfiles-prefix [prefix] (default=None)
--directories-prefix [prefix] (default=None)
--symlinks-prefix [prefix] (default=None)
--unknowns-prefix [prefix] (default=None)

--files-suffix [suffix] (default=None)
--dotfiles-suffix [suffix] (default=None)
--directories-suffix [suffix] (default="/")
--symlinks-suffix [suffix] (default=None)
--unknowns-suffix [suffix] (default=None)

Appends a string at the beggining / end of the name of this type of file.

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
    name
    size 
    extension
    creation_date
    access_date
    modification_date

Note: Everything is sorted in ascending order, use the -r 
flag to sort by desc.

#########
# Flags #
#########

# Control what to show

-sd | --show-dotfiles (default=true)
Show all files, including the ones with names starting with a ".".
Note: does not display implied "." and ".." folders.

-sb | --show-backups (default=true)
Show files with name ending with a "~".

-has | --hide-all-special (default=false) 
Hides all special filenames mentionned above.

# Control how to show

-r | --reverse (default=false)
Reverse the order of files when sorting.

-dgdf | --dont-group-directories-first (default=true)
Groups directories together before sorting them.


"#;