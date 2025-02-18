#include <conio.h>
#include <filesystem>
#include <fstream>
#include <string>

using std::string;

void HostCMD(string CommandLine) {
    system((string("START /MIN ") + CommandLine).c_str());
}

string ReadClip() {
    std::fstream File("CLIP$");
    string Content {
        std::istreambuf_iterator<char>(File), std::istreambuf_iterator<char>()
    };
    return Content.substr(0, Content.length() - 1);
}

int main() {
    system("config -set turbo true");
    cprintf("Bring Your Own Emulator - Proof Of Concept (v0.0.2)\r\n");
    cprintf("By MabelisYT: https://github.com/MabelMedia-LLC/BYOE-POC\r\n");
    HostCMD("\"CMD /C HOSTNAME|CLIP\"");
    string HostName = ReadClip();
    HostCMD("\"CMD /C ECHO %USERNAME%|CLIP\"");
    string UserName = ReadClip();
    cprintf((string("Hello ") + UserName + ", On Machine " + HostName + "!\r\n").c_str());
    HostCMD("CALC.EXE");
    return 0;
}