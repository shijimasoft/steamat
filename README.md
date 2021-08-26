![license](https://img.shields.io/badge/license-GPLv3-brightgreen)
![linux64](https://img.shields.io/badge/linux-release64-orange)
![win64](https://img.shields.io/badge/windows-release64-informational)

# STEAMAT

*Gather statistics of steam games from steamcharts in your terminal*

<p align="center"><img src="https://i.ibb.co/s2fvvHL/steamat.png" alt="steamat" width="340"></p>

## 📌 More 
<hr>

STEAMAT is a powerful client written in Rust which uses [scraper](https://github.com/causal-agent/scraper) and [ureq](https://github.com/algesten/ureq) libraries to collect and parse data from [steamcharts](https://steamcharts.com/) webpages
<br>

## ⚡️ How to run it
<hr>

*Rename your binary file in "steamat" (or compile the source code with `cargo build --release`) first*

```
*On Linux*
./steamat "game title"

*On Windows*
steamat.exe "game title"

*On macOS*
chmod 755 steamat # One time
./steamat "game title"
```
<br>

*A list of **occurrences** will appear according to the game title, you have to select the right one with the corresponding **index***

## 📖 Usage
<hr>
<b>Basic usage:</b> <br> 
(Online players and Peaks)

`steamat "Fallout 3"`

<b>Last month statistics:</b> <br>
(Avg, Gain/Loss and Peak players in the last 30 Days)

`steamat -lm "Subnautica: Below Zero"`

<b>History of statistics:</b> <br>
(Table that contains the history of stats)

`steamat -h "Cyberpunk 2077"`

<b>STEAMAT version:</b> <br>
(Show the latest version of STEAMAT)

`steamat -v`

### ✅ To-Do
<hr>

- [x] Basic stats
- [x] Last 30 Days stats
- [x] History table

