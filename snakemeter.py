def run():
    while True:
        pass

import threading
t = threading.Thread(target=run)
t.start()

import sys
import _snakemeter



_snakemeter.print_version()

print_stacktrace = _snakemeter.print_stacktrace

print_stacktrace()

print('Python:\n')
print(sys._current_frames().__len__())
for (k, v) in sys._current_frames().items():
    print (k)
    print (v)
    print('lineno = %d' % v.f_lineno)

    f = v
    while f is not None:
        print("%s:%d %s"%(f.f_code.co_filename, f.f_lineno, f.f_code.co_name))
        f = f.f_back

    # print(get_version())
