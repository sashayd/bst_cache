import matplotlib.pyplot as plt

results = {
    '1e6 insertions': dict(
        bst=[718, 605, 561, 653],
        bst_vec=[620, 542, 512, 629],
        bst_cache=[875, 412, 376, 384],
        btree_map=[668, 202, 192, 193],
        ),
    '1e6 retrievals of existing entries': dict(
        bst=[706, 441, 416, 512],
        bst_vec=[584, 541, 508, 620],
        bst_cache=[650, 348, 336, 343],
        btree_map=[582, 190, 175, 178],
        ),
    '1e6 retrievals of random entries': dict(
        bst=[757, 499, 480, 654],
        bst_vec=[653, 590, 558, 647],
        bst_cache=[627, 370, 333, 341],
        btree_map=[590, 190, 196, 200],
        ),
}

for key1, val1 in results.items():
    for key2, val2 in val1.items():
        plt.plot(list(range(len(val2))), val2, label=key2)
    plt.title(key1)
    plt.xticks(list(range(4)))
    plt.xlabel('optimization level')
    plt.ylabel('ms')
    plt.legend()
    plt.savefig(key1.replace(' ', '_')+'.png')
    plt.clf()

