#include <stdio.h>

int main(){
    int a;
    float b;
    double c;
    char name[6];

    puts("Pointer to all the variables declared: ");
    printf("a: %p: \n, &a");
    printf("b: %p: \n, &b");
    printf("c: %p: \n, &c");
    printf("name[0]: %p: \n", name);

    for (int i = 0; i<6; i++){
        printf("name: %p: \n", name);
    }

    scanf("%s", &name);
    printf("%s\n", name);

   // int *p2a;
   // p2a = &a;

  //  printf("%p\n", p2a);

}