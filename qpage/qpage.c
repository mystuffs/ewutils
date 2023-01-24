#include <stdio.h>
#include <string.h>
#include <err.h>
#include <getopt.h>

#define DEF '#'
#define H1  "#"
#define H2  "##"
#define H3  "###"
#define H4  "####"
#define H5  "#####"
#define H6  "######"

static void add_head(int pre)
{
    if (pre)
        fprintf(stdout,
            "<!DOCTYPE HTML>\n"
            "<html>\n"
            "<pre>\n"
        );
    else
        fprintf(stdout,
            "<!DOCTYPE HTML>\n"
            "<html>\n"
        );
}

static void add_foot(int pre)
{
    if (pre)
        fprintf(stdout,
            "</pre>\n"
            "</html>\n"
        );
    else
        fprintf(stdout,
            "</html>\n"
        );
}

static void parse_hashtag(char *src, unsigned int pos, int pre)
{
    if (src[pos] == DEF && src[pos + 1] != DEF)
        fprintf(stdout, "<h1>%s</h1>\n", strtok(strtok(src, "\n"), H1));
    else if (src[pos] == DEF && src[pos + 1] == DEF &&
            src[pos + 2] != '#')
        fprintf(stdout, "<h2>%s</h2>\n", strtok(strtok(src, "\n"), H2));
    else if (src[pos] == DEF && src[pos + 1] == DEF &&
            src[pos + 2] == DEF)
        fprintf(stdout, "<h3>%s</h3>\n", strtok(strtok(src, "\n"), H3));
    else if (src[pos] == DEF && src[pos + 1] == DEF &&
            src[pos + 2] == DEF && src[pos + 3] == DEF)
        fprintf(stdout, "<h4>%s</h4>\n", strtok(strtok(src, "\n"), H4));
    else if (src[pos] == DEF && src[pos + 1] == DEF &&
            src[pos + 2] == DEF && src[pos + 3] == DEF &&
            src[pos + 4] == DEF)
        fprintf(stdout, "<h5>%s</h5>\n", strtok(strtok(src, "\n"), H5));
    else if (src[pos] == DEF && src[pos + 1] == DEF &&
            src[pos + 2] == DEF && src[pos + 3] == DEF &&
            src[pos + 4] == DEF && src[pos + 5] == DEF)
        fprintf(stdout, "<h6>%s</h6>\n", strtok(strtok(src, "\n"), H6));
    else
        if (pre)
            fprintf(stdout, "<pre>%s</pre>\n", src);
        else
            fprintf(stdout, "<p>%s</p>\n", src);
}

static void conv(char *file, int pre)
{
    FILE *fp = fopen(file, "rb");
    char line[500];
    if (fp == NULL)
        err(1, "fopen()");

    int i = 0;
    while (fgets(line, sizeof(line), fp)) {
        parse_hashtag(line, i, pre);
    }
}

int main(int argc, char **argv)
{
    if (argc < 2) {
        fprintf(stderr, "error: please specify the file\n");
        return 1;
    }
    if (!strcmp(argv[1], "-p")) {
        if (argc < 3) {
            fprintf(stderr, "error: please specify the file\n");
            return 1;
        }
        add_head(1);
        conv(argv[2], 1);
        add_foot(1);
    } else if (!strcmp(argv[1], "-t")) {
        if (argc < 3) {
            fprintf(stderr, "error: please specify the file\n");
            return 1;
        }
        add_head(0);
        conv(argv[2], 1);
        add_foot(0);
    } else {
        add_head(0);
        conv(argv[1], 0);
        add_foot(0);
    }
}
