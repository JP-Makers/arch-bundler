#! /bin/bash

#Colors
RPINK='\033[1;38;5;125m'
RBLUE='\033[1;38;5;025m'
RB='\033[1;38;5;130m'
RDB='\033[1;38;5;159m'
RYELLOW='\033[1;38;5;003m'
R='\033[1;38;5;219m'
RED='\033[1;31m'
GREEN='\033[0;32m'
RGREEN='\033[1;38;5;037m'
BLUE='\033[1;34m'
CYAN='\033[1;36m'
PURPLE='\033[1;35m'
YELLOW='\033[1;33m'
WHITE='\033[1;37m'
NC='\033[0m'

#Input
if [ -z "$1" ]
then
    echo -e "${BLUE}Usage:${NC} ${WHITE}$0${NC} ${RED}<input_video>${NC}"
    exit 1
fi

# Function to center text
center() {
    termwidth=$(tput cols)
    textwidth=${#1}
    padding=$(( (termwidth - textwidth) / 2 ))
    printf "%*s%s\n" $padding "" "$1"
}

#Banner Function
banner() {
    clear
    echo -e $RPINK
    echo
    center " ▄▄▄██▀▀ ██▀███   ▄▄▄██▀▀"
    center "   ▒██  ▓██ ▒ ██▒   ▒██  "
    center "   ░██  ▓██ ░▄█ ▒   ░██  "
    center "▓██▄██▓ ▒██▀▀█▄  ▓██▄██▓ "
    center " ▓███▒  ░██▓ ▒██▒ ▓███▒  "
    center " ▒▓▒▒░  ░ ▒▓ ░▒▓░ ▒▓▒▒░  "
    center " ▒ ░▒░    ░▒ ░ ▒░ ▒ ░▒░  "
    center " ░ ░ ░     ░   ░  ░ ░ ░  "
    center " ░   ░     ░      ░   ░  "
    echo -e $NC
}

music() {
    clear
    echo -e $RPINK
    echo
    center "                    ▒▒                                   "
    center "        ▒▒          ▒▒▒▒                        ▒▒       "
    center "        ▒▒▒▒        ▒▒▒▒▒▒        ▒▒            ▒▒▒▒     "
    center "        ▒▒▒▒▒▒      ▒▒  ▒▒        ▒▒▒▒          ▒▒▒▒▒▒   "
    center "        ▒▒▓▓▒▒▓▓    ▒▒            ▒▒▒▒▒▒        ▒▒██▒▒██ "
    center "        ▒▒  ▓▓▓▓    ▒▒            ▒▒▓▓▒▒▓▓      ▒▒  ████ "
    center "        ▒▒    ▓▓    ▒▒            ▒▒  ▓▓▓▓      ▒▒    ██ "
    center "        ▒▒          ▒▒            ▒▒    ▓▓      ▒▒       "
    center "        ▒▒    ▒▒▒▒▒▒▒▒            ▒▒            ▒▒       "
    center "  ▒▒▒▒▒▒▒▒  ▒▒  ▒▒▒▒              ▒▒      ▒▒▒▒▒▒▒▒       "
    center "▒▒  ▒▒▒▒▓▓  ▒▒▒▒▒▒▒▒        ▒▒▒▒▒▒▒▒    ▒▒  ▒▒▒▒██       "
    center "▒▒▒▒▒▒▒▒▓▓                ▒▒  ▒▒▒▒▓▓    ▒▒▒▒▒▒▒▒██       "
    center "  ▓▓▓▓▓▓                  ▒▒▒▒▒▒▒▒▓▓      ██████         "
    center "                            ▓▓▓▓▓▓                       "
    echo -e $NC
}

banner

echo -e $RBLUE
#Messages
center "1. Extract Audio        "
center "2. Add Audio            "
center "3. Default Audio        "
center "4. Unwanted Audio Remove"
center "5. Extract Subtitle     "
center "6. Add Subtitle         "
center "7. Default Subtitle     "
center "8. MKV to MP4           "
echo -e $NC

#Input 
echo -e "${RB}Enter the Value :${NC} ${RDB}\c${NC}"
read MSG

#File Location
file() {
echo
echo -e "${RYELLOW}$PWD${NC}"
echo -e $R
ls
echo -e $NC
}

file

#Variables
INPUT="$1"

#Input Found Condition
if [ -e $INPUT ]
then
    echo -e "${RGREEN}$INPUT found${NC}"
    echo
else
    echo -e "${RED}$INPUT not found${NC}"
    exit
fi

#Input Found Funtion
input() {
    if [ -e $SOURCE ]
    then
        echo -e "${RGREEN}$SOURCE found${NC}"
        echo
    else
        echo -e "${RED}$SOURCE not found${NC}"
        exit
    fi
}

#Output Function
output() {
    echo -e "${RB}Enter the Output File Name :${NC} ${RDB}\c${NC}"
    read OUT
    echo
    echo -e "${RB}Enter the Output File Type :${NC} ${RDB}\c${NC}"
    read PUT

    #Variables
    OUTPUT="$OUT.$PUT"
    END="${RGREEN}Conversion completed: $OUTPUT${NC}"
}

#DATA Funtion
data() {
    x=$IN
    y=1
    DATA=$(( x - y ))
}

#Main Funtions
extract_audio() {
    music
    echo
    echo -e "${RB}Select Audio :${NC} ${RDB}\c${NC}"
    read IN
    data
    echo
    echo -e "${RB}Enter the Bitrate ${RGREEN}eg.(192k ,320k)${NC} :${NC} ${RDB}\c${NC}"
    read BIT
    echo
    output
    echo
    echo -e "${RPINK}ffmpeg -i $INPUT -map 0:a:$DATA -ab $BIT $OUTPUT${NC}"
    echo -e $GREEN
    ffmpeg -i $INPUT -map 0:a:$DATA -ab $BIT $OUTPUT
    echo -e $NC
    echo -e "$END"
}

add_audio (){
    music
    echo
    file
    echo -e "${RB}Enter the Audio File Name With Type ${RGREEN}eg.(File.mp3 ,File.ac3)${NC} :${NC} ${RDB}\c${NC}"
    read SOURCE
    echo
    output
    echo
    input
    echo -e "${RPINK}ffmpeg -i $INPUT -i $SOURCE -map 0 -map 1:a:0 -c copy $OUTPUT${NC}"
    echo -e $GREEN
    ffmpeg -i $INPUT -i $SOURCE -map 0 -map 1:a:0 -c copy $OUTPUT
    echo -e $NC
    echo -e "$END"
}

default_audio() {
    music
    echo
    echo -e "${RB}Select Default Audio :${NC} ${RDB}\c${NC}"
    read IN
    data
    echo
    output
    echo
    echo -e "${RPINK}ffmpeg -i $INPUT -map 0 -c copy -disposition:a none -disposition:a:$DATA default $OUTPUT${NC}"
    echo -e $GREEN
    ffmpeg -i $INPUT -map 0 -c copy -disposition:a none -disposition:a:$DATA default $OUTPUT
    echo -e $NC
    echo -e "$END"
}

unwanted_audio() {
    music
    echo
    echo -e "${RB}Select Remove Audio :${NC} ${RDB}\c${NC}"
    read IN
    data
    echo
    output
    echo
    echo -e "${RPINK}ffmpeg -i $INPUT -map 0 -map -0:a:$DATA -c copy $OUTPUT${NC}"
    echo -e $GREEN
    ffmpeg -i $INPUT -map 0 -map -0:a:$DATA -c copy $OUTPUT
    echo -e $NC
    echo -e "$END"
}

extract_srt() {
    music
    echo
    echo -e "${RB}Select Subtitle :${NC} ${RDB}\c${NC}"
    read IN
    data 
    echo
    output
    echo
    echo -e "${RPINK}ffmpeg -i $INPUT -map 0:s:$DATA $OUTPUT${NC}"
    echo -e $GREEN
    ffmpeg -i $INPUT -map 0:s:$DATA $OUTPUT
    echo -e $NC
    echo -e "$END"
}

add_srt() {
    music
    echo
    file
    echo -e "${RB}Enter the Subtitle File Name With Type ${RGREEN}eg.(File.srt ,File.vtt)${NC} :${NC} ${RDB}\c${NC}"
    read SOURCE
    echo
    output
    echo
    input
    echo -e "${RPINK}ffmpeg -i $INPUT -i $SOURCE -map 0 -map 1 -c copy -c:s srt -disposition:s:0 default $OUTPUT${NC}"
    echo -e $GREEN
    ffmpeg -i $INPUT -i $SOURCE -map 0 -map 1 -c copy -c:s srt -disposition:s:0 default $OUTPUT
    echo -e $NC
    echo -e "$END"
}

default_srt() {
    music
    echo
    echo -e "${RB}Select Default Subtitle :${NC} ${RDB}\c${NC}"
    read IN
    data
    echo
    output
    echo
    echo -e "${RPINK}ffmpeg -i $INPUT -map 0 -c copy -disposition:s none -disposition:s:$DATA default $OUTPUT${NC}"
    echo -e $GREEN
    ffmpeg -i $INPUT -map 0 -c copy -disposition:s none -disposition:s:$DATA default $OUTPUT
    echo -e $NC
    echo -e "$END"
}

#Condition
if [ $MSG -eq 1 ]
then 
    extract_audio
elif [ $MSG -eq 2 ]
then 
    add_audio
elif [ $MSG -eq 3 ]
then 
    default_audio
elif [ $MSG -eq 4 ]
then 
    unwanted_audio
elif [ $MSG -eq 5 ]
then 
    extract_srt
elif [ $MSG -eq 6 ]
then 
    add_srt
elif [ $MSG -eq 7 ]
then 
    default_srt
elif [ $MSG -eq 8 ]
then
    echo -e $GREEN
    for f in *.mkv; do ffmpeg -i "$f" -c copy "${f%.mkv}.mp4"; done
    echo -e $NC
    echo -e "${RGREEN}Conversion completed${NC}"
else
    echo -e "${RED}Something Went Wrong${NC}"
fi
