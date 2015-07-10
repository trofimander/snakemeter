import _snakemeter

class Sampler(object):
    def __init__(self, rate=1000):
        self.rate = rate

    def start(self):
        _snakemeter.start_sampling(self, self.rate)

    def stop(self):
        _snakemeter.stop_sampling(self)

    def reset(self):
        _snakemeter.reset_sampling(self)

    def get_stats(self):
        stats = _snakemeter.get_sampling_stats(self)
        return interpret_stats(stats)


class Stats(object):
    pass

def interpret_stats(stats):
    return Stats()

