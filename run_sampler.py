def loop():
    while True:
        pass

def run_sampling():
    import snakemeter

    sampler = snakemeter.Sampler(1)

    sampler.start()

    import threading
    t = threading.Thread(target=loop)
    t.start()


    import time
    time.sleep(3)


    sampler.stop()

    print(sampler._sampler)

    print(sampler.get_stats())



if __name__ == '__main__':
    run_sampling()