@echo off
if "%~x1"==".xml" (param-xml asm %1 -o %~n1.prc -l ParamLabels.csv) else (param-xml disasm %1 -o %1.xml -l ParamLabels.csv)

pause