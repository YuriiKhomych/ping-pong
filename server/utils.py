import logging


def get_logger(name: str) -> logging.Logger:
    logger = logging.getLogger(name)
    logger.setLevel(logging.DEBUG)
    logger.addHandler(logging.StreamHandler())
    formatter = logging.Formatter(
        "%(asctime)s  %(message)s", datefmt="%H:%M:%S"
    )
    logger.handlers[0].setFormatter(formatter)
    return logger
