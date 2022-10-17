BASE_OFFSET = 0x4E00


class Builder:
    raw = None
    buff = 0x00
    out = []

    def __init__(self):
        pass

    def build(self, raw_data):
        self.raw = raw_data
        for i in range(len(self.raw)):
            while (self.buff != 0):
                self.buff = self.raw[i]
        pass
