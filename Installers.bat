@echo off
cargo build --release || exit /b 1
set W=C:\Program Files (x86)\WiX Toolset v3.11\bin
cd installer
"%W%\candle.exe" -nologo -arch x64 -out ..\target\aof.wixobj aof-ekampus.wxs || exit /b 1
"%W%\light.exe" -nologo -out ..\dist\AOF-e-Kampus-1.0.0-x64.msi ..\target\aof.wixobj || exit /b 1
"C:\Program Files (x86)\NSIS\makensis.exe" aof-ekampus.nsi || exit /b 1
del ..\dist\*.wixpdb
cd ..
