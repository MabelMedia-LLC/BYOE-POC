#include <conio.h>
#include <filesystem>
#include <fstream>
#include <string>

using std::string;

void HostCMD(string CommandLine) {
    system((string("START ") + CommandLine + " /MIN").c_str());
}

string ReadClip() {
    std::fstream File("CLIP$");
    string Content {
        std::istreambuf_iterator<char>(File), std::istreambuf_iterator<char>()
    };
    return Content.substr(0, Content.length() - 1);
}

int main() {
    HostCMD("\"CMD /C HOSTNAME|CLIP\"");
    string HostName = ReadClip();
    HostCMD("\"CMD /C ECHO %USERNAME%|CLIP\"");
    string UserName = ReadClip();
    cprintf((string("Hello ") + UserName + ", On Machine " + HostName + "!\r\n").c_str());
    return 0;
}