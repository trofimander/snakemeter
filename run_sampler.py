def loop():
    while True:
        pass

def run_sampling():
    import snakemeter

    sampler = snakemeter.Sampler(500)

    sampler.start()

    import threading
    t = threading.Thread(target=loop)
    t.start()


    import time
    time.sleep(3)


    sampler.stop()

    sampler.print_stats()



if __name__ == '__main__':
    run_sampling()