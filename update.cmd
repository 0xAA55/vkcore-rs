@echo off
cd vkcore-parse
python vkparse.py
cd ..
copy vkcore-parse\vkcore.rs src\vkcore.rs
