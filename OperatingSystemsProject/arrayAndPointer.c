#include <stdio.h>

int main(){
    char name[5] = {
        'H', 
        'E', 
        'N', 
        'R', 
        'Y'
    };

    int rowMatrics[4] = {2,5,6,6};

    int matrix[3][2] = {
        {1, 7}, {3, 9}, {5, 10}
    };
    printf("%i", matrix[1][1]);
    printf("%s", name);

    return 0;
}