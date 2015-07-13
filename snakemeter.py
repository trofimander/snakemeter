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
        return _snakemeter.get_sampling_stats(self)

    def get_stats_as_pstats(self):
        return convert_to_pstats


def convert_to_pstats(stats):
    from collections import defaultdict


    import pstats
    class _PStatHolder:
        def __init__(self, d):
            self.stats = d
        def create_stats(self):
            pass

    def pstat_id(fs):
        return (fs.module, fs.lineno, fs.name)

    _pdict = {}

    # convert callees to callers
    _callers = defaultdict(dict)
    for fs in stats:
        for ct in fs.children:
            _callers[ct][pstat_id(fs)] = (ct.ncall, ct.nactualcall, ct.tsub ,ct.ttot)

    # populate the pstat dict.
    for (path, ) in stats.callable_stats:
        _pdict[pstat_id(fs)] = (fs.ncall, fs.nactualcall, fs.tsub, fs.ttot, _callers[fs], )

    return pstats.Stats(_PStatHolder(_pdict))



def interpret_stats(stats):
    return Stats()
