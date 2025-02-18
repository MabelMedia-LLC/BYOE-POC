# BYOE-POC
Bring Your Own Emulator (BYOE) - A POC For Using DOSBox For Cloaking Malware Files Under Windows.

# Q & A
Q: Why DOSBox?
A: We Use DOSBox So All Malware Files Can Live Inside A Virtual Disk Image.
The Files Not Touching Disk Helps Evade AV / EDR (No Direct Files On Disk) For A Pseudo-Fileless Malware.
Also, The Main Payload Being A DOS/32 Executable Helps Confuse AV / EDR As Well (No PE/NT Headers).
Q: Why DOSBox-X?
A: Standard DOSBox Requires 2 DLLs in Addition To It's Main Exec, And DOSBox-x Doesn't (Reduces File Count & Code Complexity).

Signiture Rules Welcome! (Please Open PRs Adding Them, As I'm Not Familliar With Them).