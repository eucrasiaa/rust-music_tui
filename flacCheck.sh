#!/usr/bin/env bash

# misc notes:
# printf "\e[?1049h" - switch alt buffer, so you cna print w/o junking up terminal
# "\e[?1049l  - exit mode
#
# kitty specific graphical protocalls beyond just ansi and xterm-256color
#  - 1. we can use kitty's graphics protocol to display the album art, which is a nice touch for a music file. This can be done using the `kitty +kitten icat` command, which allows us to display images directly in the terminal.
#  - 2. we can also use kitty's graphics protocol to display a progress bar or other visual indicators 

# Truecolor / 24-bit color, using:
#     \e[38;2;<r>;<g>;<b>m for foreground
#     \e[48;2;<r>;<g>;<b>m for background.
#
#Sixel Graphics? 
# Block + braille rendering charas
#
#   Block Elements: U+2580–U+259F (▀▄█ etc.) — half/quarter blocks
#   Braille Patterns: U+2800–U+28FF — CAVA uses these for the smooth bar effect
#   Box Drawing: U+2500–U+257F — for TUI borders/layouts
# ansi escapes
# "XTerm Control Sequence" -
# "Kitty keyboard protocol" 
#
# tput?
# take advantage of ffmpeg, ffprobe, flac, and metaflac to generate an integrety, error, and metadata report for a given flac file, using tput or nice formatting. also, check 
