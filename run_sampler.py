def run_sampling():
    import snakemeter

    sampler = snakemeter.Sampler(100)

    sampler.start()


if __name__ == '__main__':
    run_sampling()