# NAME

Try, One Seek, One Read

# DESCRIPTION

Test how performance One Seek, One Read vs. Normal reading.

# RESULT

Count system call by `dtruss -t syscall -f -c`.
Time by `time` command.

iteration count is set to: 1000

| name          | open | madvise | seek | read | time [ms]  |
+---------------+------+---------+------+------+------------+
| archive(r, o) |    3 |     423 |  954 |  962 | 67 to 80   |
| archive(r, n) |    3 |     423 |  954 |  962 | 67 to 80   |
| normal(r)     |  148 |      67 |    0 | 1849 | 177 to 193 |
| one_read(r)   |  708 |     307 |    0 |  713 | 80 to 109  |
| archive(s, o) |    3 |     458 |    0 | 1008 | 62 to 68   |
| archive(s, n) |    3 |     448 |  941 |  948 | 68 to 71   |
| normal(s)     |  143 |      99 |    0 | 1825 | 168 to 199 |
| one_read(s)   |  704 |     317 |    0 |  709 | 78 to 92   |

- archive(r, o): one seek, one read | random, seek optimization
- archive(r, n): one seek, one read | random, no seek optimization
- normal(r): normal way | random
- one_read(r): normal way | random, one read
- archive(s, o): one seek, one read | sequential, seek optimization
- archive(s, n): one seek, one read | sequential, no seek optimization
- normal(s): normal way | sequential
- one_read(s): normal way | sequential, one read

environment:
- os: macOS Catalina
- cpu: 1.4 GHz (Intel Core i7)
- memory: 16 GB 1867 MHz LPDDR3
- storage: SSD

# REFERENCES

- [SCHOOL GIRL STRIKERS's Engine](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=2&cad=rja&uact=8&ved=2ahUKEwipsKms9-nnAhUU_GEKHUAWBiEQFjABegQIAhAB&url=http%3A%2F%2Fwww.jp.square-enix.com%2Fconference%2F2014%2Ftechnical_seminar%2Fimg%2Fpdf%2FSQEX_DevCon_sugimoto.pdf&usg=AOvVaw33gWJoLD9Ww_4NWlg5di38)
