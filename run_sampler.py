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


    while True:
        pass




if __name__ == '__main__':
    run_sampling()