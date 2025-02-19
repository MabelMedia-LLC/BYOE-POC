#include <conio.h>
#include <filesystem>
#include <fstream>
#include <string>
#include <unistd.h>

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
    HostCMD("\"CMD /C ECHO %USERNAME%|CLIP\"");
    chdir("A:\\");
    UserName = ReadClip();
    system((string("MOUNT B: C:\\Users\\") + UserName + "\\AppData\\Local\\Temp").c_str());
}

int main() {
    cprintf("Bring Your Own Emulator - Proof Of Concept (v0.0.3)\r\n");
    cprintf("By MabelisYT: https://github.com/MabelMedia-LLC/BYOE-POC\r\n");
    Setup();
    while(true) {
        if(!std::filesystem::exists("B:\\RuntimeMessage.dat")) {
            system("RESCAN /Q B:");
            sleep(1);
            continue;
        }
        std::fstream File("B:\\RuntimeMessage.dat");
        string Content {
            std::istreambuf_iterator<char>(File), std::istreambuf_iterator<char>()
        };
        std::filesystem::remove("B:\\RuntimeMessage.dat");
        char TypeCode = Content[0];
        switch (TypeCode) {
            case 0: {
                unsigned short Length = Content[1] | Content[2] << 8;
                string Command = Content.substr(3, Length);
                system(Command.c_str());
                break;
            }
            case 1: {
                return 0;
            }
            default: continue;
        } 
    }
}