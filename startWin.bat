@echo off
python3 -m venv venv
call venv\Scripts\activate.bat
venv\bin\pip install -r requirements.txt && venv\bin\python3 main.py
pause