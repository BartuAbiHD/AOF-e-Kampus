@echo off
color a
cls
title AOF Ekampus build al
cargo build --release
if exist targeteleaseof-ekampus.exe copy /Y targeteleaseof-ekampus.exe "AOF e-Kampus.exe"
pause
