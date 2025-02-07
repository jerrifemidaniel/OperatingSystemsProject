#include <stdio.h>

float addition(float a, float b){
    return a + b;
}

float subtraction(float a, float b){
    return a - b;
}

float multiplication(float a, float b){
    return a * b;
}

float divition(float a, float b){
    return a / b;
}

int main(){
  
   printf("Input your a and b");
   printf("What function do you want to do?");
   
    float a;
    float b;

   int e = 1;
   int f = 2;
   int g = 3;
   int h = 4;

   printf("Select 1, 2, 3, 4; for addition, subtraction, division and multiplication respectively");

   printf("Input your a and b respectively");
   scanf("%i", &a);
   scanf("%i",&b);

   int opt;
   scanf("%i", &opt);

    if(opt = e){
        addition(a, b);
    }else if (opt = f)
    {
        subtraction(a, b);
    }else if(opt = g){
        multiplication(a, b);
    }else if(opt = h){
        divition(a, b);
    }
    

    return 0;
}