#include<stdio.h>
#include<signal.h>
#include<stdlib.h>
#include<stdbool.h>

static volatile int running_processes = 0;

void handler(int sig) {
    while (waitpid(-1, NULL, WNOHANG) > 0) {
        running_processes -= 1;
    }
}

int main() {
    signal(SIGCHLD, handler);
    const int num_processes = 10;
    for (int i = 0; i < num_processes; i++) {
        if (fork() == 0) {
            sleep(1);
            exit(0);
        }
        running_processes += 1;
        printf("%d running processes\n", running_processes);
    }
    while(running_processes > 0) {
        pause();
    }
    printf("All processes exited! %d running processes\n", running_processes);
    return 0;
}
