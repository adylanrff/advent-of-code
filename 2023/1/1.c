#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void appendChar(char *str, char c)
{
    size_t len = strlen(str);
    snprintf(str + len, sizeof(str) - len, "%c", c);
}

int get_number_from_string(char *num_string)
{
    char *numbers[] = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

    for (int i = 0; i < 10; i++)
    {
        if (strstr(num_string, numbers[i]) != NULL)
        {
            return i;
        }
    }

    return -1;
}

int main(int argc, char const *argv[])
{
    FILE *fp;
    fp = fopen("./data.txt", "rb");
    char *line = NULL;
    size_t len = 0;

    if (fp == NULL)
    {
        return 1;
    }

    long long int total = 0;

    int line_num = 0;

    while ((getline(&line, &len, fp)) != -1)
    {

        int first_num = -1;
        int second_num = -1;

        char first_num_str[1000] = "";
        char second_num_str[1000] = "";

        int i = 0;
        while (i < len && (line[i] < '0' || line[i] > '9') && get_number_from_string(first_num_str) == -1)
        {
            sprintf(first_num_str, "%s%c", first_num_str, line[i]);
            i++;
        }

        if (get_number_from_string(first_num_str) != -1)
        {
            first_num = get_number_from_string(first_num_str);
        }
        else if (line[i] >= '0' && line[i] <= '9')
        {
            first_num = line[i] - '0';
        }

        printf("%d: first_num_str: %s | first_num: %d | ", line_num, first_num_str, first_num);

        int j = strlen(line) - 1;
        while (j >= 0 && (line[j] < '0' || line[j] > '9') && get_number_from_string(second_num_str) == -1)
        {
            sprintf(second_num_str, "%c%s", line[j], second_num_str);
            j--;
        }

        if (get_number_from_string(second_num_str) != -1)
        {
            second_num = get_number_from_string(second_num_str);
        }
        else if (line[j] >= '0' && line[j] <= '9')
        {
            second_num = line[j] - '0';
        }

        printf("second_num_str: %s | second_num: %d\n", second_num_str, second_num);

        int curline = (first_num * 10) + second_num;
        total += curline;
        line_num++;
    }

    printf("%lld", total);

    fclose(fp);
    /* code */
    return 0;
}