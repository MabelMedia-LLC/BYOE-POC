#include <conio.h>
#include <filesystem>
#include <fstream>
#include <string>

using std::string;

string UserName;

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

void Setup() {
    system("config -set turbo true");
    HostCMD("\"CMD /C ECHO %USERNAME%|CLIP\"");
    UserName = ReadClip();
    system((string("MOUNT B: C:\\Users\\") + UserName + "\\AppData\\Local\\Temp").c_str());
}

int main() {
    cprintf("Bring Your Own Emulator - Proof Of Concept (v0.0.2)\r\n");
    cprintf("By MabelisYT: https://github.com/MabelMedia-LLC/BYOE-POC\r\n");
    Setup();
    while(true) {
        if(!std::filesystem::exists("B:\\RuntimeMessage.bin")) continue;
        std::fstream File("B:\\RuntimeMessage.bin");
    }
}