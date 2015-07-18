import _snakemeter
import os
import sys

def save_main_module(file, module_name):
    sys.modules[module_name] = sys.modules['__main__']
    sys.modules[module_name].__name__ = module_name
    from imp import new_module

    m = new_module('__main__')
    sys.modules['__main__'] = m
    if hasattr(sys.modules[module_name], '__loader__'):
        setattr(m, '__loader__', getattr(sys.modules[module_name], '__loader__'))
    m.__file__ = file

    return m


class Sampler(object):
    def __init__(self, rate=1000):
        self.rate = rate

    def start(self):
        _snakemeter.start_sampling(self, self.rate)

    def stop(self):
        _snakemeter.stop_sampling(self)

    def reset(self):
        _snakemeter.reset_sampling(self)

    def _get_stats(self):
        return _snakemeter.get_sampling_stats(self)

    def print_stats(self):
        Statistics(self._get_stats()).print_stats()

    def run(self, filename):
        m = save_main_module(file, 'run_profiler')
        globals = m.__dict__
        try:
            globals['__builtins__'] = __builtins__
        except NameError:
            pass  # Not there on Jython...

        self.start()

        try:
            exec(compile(open(filename, "rb").read(), filename, 'exec'), globals, None)
        finally:
            self.stop()
            self.print_stats()


class Statistics(object):
    def __init__(self, stats):
        self.total_time = stats.total_time
        self.samples_count = stats.samples_count
        self.callable_stats = [CallableStats(path, name, line, cum_count, self_count, stats.total_time, stats.samples_count) for (path, name, line, cum_count, self_count) in stats.callable_stats]

    def print_stats(self):
        l = self.callable_stats[:]
        l.sort(reverse=True, key=lambda x: x.total_ms)

        print ('%5.5s %10.10s   %7.7s  %30.30s  %8.8s' %
                      ('%  ', 'total', 'self', 'callable', 'filename'))
        print ('%5.5s  %9.9s  %8.8s  %8.8s  %-8.8s' %
                      ('time', 'seconds', 'seconds', '', ''))

        for x in l:
            x.display()


class CallableStats(object):
    def __init__(self, file, name, line, cum_count, self_count, total_time, total_samples):
        self.self_samples = self_count
        self.total_samples = total_samples

        sample_ms = total_time / total_samples / 1000000.0

        print(total_time)
        self.file = file
        self.filename = os.path.basename(file)
        self.callable = name
        self.line = line
        self.name = "%s:%s" % (file, line)
        self.percent = 1.0 * self_count / total_samples * 100
        self.total_ms = (cum_count + self_count) * sample_ms
        self.self_ms = self_count * sample_ms

    def display(self):
        print ('%6.2f %9.2f %9.2f  %30.30s  %s' % (self.percent,
                                                 self.total_ms/1000.0,
                                                 self.self_ms/1000.0,
                                                 self.callable, self.name))

if __name__ == '__main__':
    file = sys.argv[1]

    del sys.argv[0]

    sampler = Sampler()
    sampler.run(file)