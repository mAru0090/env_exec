@echo off
add_paths "%USERPROFILE%/paths.txt" powershell -NoExit -Command "Set-Location -Path '%USERPROFILE%'"
