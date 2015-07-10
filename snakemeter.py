import _snakemeter

class Sampler(object):
    def __init__(self, rate=1000):
        self.rate = rate

    def start(self):
        _snakemeter.start_sampling(self.rate)

    def stop(self):
        _snakemeter.stop_sampling()

    def reset(self):
        _snakemeter.reset_sampling()

    def get_stats(self):
        stats = _snakemeter.get_sampling_stats()
        return interpret_stats(stats)


class Stats(object):
    pass

def interpret_stats(stats):
    return Stats()

