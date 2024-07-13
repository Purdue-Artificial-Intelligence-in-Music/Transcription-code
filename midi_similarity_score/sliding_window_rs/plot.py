import pickle
import matplotlib.pyplot as plt
import numpy as np

f = ["pred", "target"]
for f in f:
    with open("pickle", "rb") as f:
        im = pickle.load(f)
        im = np.array(im)
        plt.figure()
        i = plt.imshow(im.T, aspect="auto")
        i.axes.invert_yaxis()
plt.show()
