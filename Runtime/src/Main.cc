#include <conio.h>
#include <string>

void HostCMD(std::string CommandLine) {
    system((std::string("START ") + CommandLine).c_str());
}

int main() {
    cprintf("Launching Host Process (calc.exe)...\r\n");
    HostCMD("calc.exe");
    return 0;
}