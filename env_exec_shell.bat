@echo off
env_exec "%USERPROFILE%/env_exec_config.toml" powershell -NoExit -Command "Set-Location -Path '%USERPROFILE%'"
