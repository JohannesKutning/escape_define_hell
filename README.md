# Escape Define Hell

In case you have to work with an external library that comes right out of the
define hell.  Escape define hell is a command line tool that shows you which
defines are required to activate a line of code.

## Usage


    escape_define_hell --input <FILE> --line <INTEGER>


Select your source file with the **-i/--input** argument and the line number with
the **-l/--line** argument.

A report containing line number and all  **#ifdef**, **#ifndef**, **#if** and
**#else** that are required to activate the selected line will be generated.
E.g. calling **escape_define_hell** with the test file returns the following
report.  Each line contains the line number from the file followed by a colon
and the lines content.


    escape_define_hell -i test/test.c -l 13


     2: #ifdef def0
     4: #ifdef def1
     6: #ifdef def2
     8: #ifdef def3
    10: #else
    12: #ifdef def4
    13: // line 13


