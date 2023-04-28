import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import sys
from matplotlib import animation
import matplotlib.style as mplstyle


def charge_color(c):
    if c>0:
        return 'red'
    elif c<0:
        return 'blue'
    else:
        return 'gray'

def run(f_in,f_out,desc):
    mplstyle.use('fast')
    print("START PYTHON",desc)
    print("READ DATA",desc)
    df = pd.read_csv(f_in, sep=" ", header=None)
    e1c = float(df[0][0])
    e1p = float(df[1][0])
    e2c = float(df[2][0])
    e2p = float(df[3][0])
    e3c = float(df[4][0])
    e3p = float(df[5][0])

    b1c = float(df[6][0])
    b1p = float(df[7][0])
    b2c = float(df[8][0])
    b2p = float(df[9][0])
    b3c = float(df[10][0])
    b3p = float(df[11][0])
    df = df[1:]

    values = []
    charges = []

    print("PARSE DATA",desc)

    for i in range(1,len(df[1])):
        if int(df[0][i].replace(':',''))+1 > len(values):
            values.append([])
        
        values[int(df[0][i].replace(':',''))].append(df[1][i].replace('(','').replace(')','').split(','))


    for i in range(len(values)):
        for j in range(len(values[i])):
            for k in range(len(values[i][j])):
                values[i][j][k] = float(values[i][j][k])


    charges = [int(i) for i in df[2][0:len(values)]]

    fig = plt.figure(figsize=(24,18))
    ax = fig.add_subplot(121,projection='3d')

    print("PLOT DATA.",desc)
    points = [ax.plot([values[i][0][0]], [values[i][0][1]], [values[i][0][2]], 'o', color=charge_color(charges[i]), label='{}'.format(i)) for i in range(len(values))]
    texts = [ax.text2D(1, 0.97-i*0.025,  '({:.2f},{:.2f},{:.2f})'.format(values[i][0][0],values[i][0][1],values[i][0][2]), transform=ax.transAxes) for i in range(len(values))]
    plt.legend()
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_zlabel('Z')

    ax.set_xlim((-10,10))
    ax.set_ylim((-10,10)) 
    ax.set_zlim((-10,10))

    ratio=10
    maximal = max([len(i) for i in values])
    n_values = [n for n in range(maximal) if n % ratio == 0 ]

    def update_point(n, values, points):
        ps = []
        for i in range(len(points)):
            if n < len(values[i]):
                p, = points[i]
                p.set_data(np.array([values[i][n][0]]),np.array([values[i][n][1]]))
                p.set_3d_properties(values[i][n][2], 'z')
                texts[i].set_text('({:.2f},{:.2f},{:.2f})'.format(values[i][n][0],values[i][n][1],values[i][n][2]))
                ps.append(p)
        return tuple(ps)

    ax = fig.add_subplot(122,projection='3d')
    # Create a grid of points in x, y, and z directions
    x, y, z = np.meshgrid(np.linspace(-10, 10, 5),
                        np.linspace(-10, 10, 5),
                        np.linspace(-10, 10, 5))

    # Define a vector field function
    def e_field(x, y, z):
        return np.array([e1c*x**e1p, e2c*y**e2p, e3c*y**e3p])

    def b_field(x,y,z):
        return np.array([b1c*x**b1p, b2c*y**b2p, b3c*y**b3p])

    # Evaluate the vector field at each point in the grid
    u, v, w = e_field(x, y, z)
    ax.quiver(x, y, z, u, v, w, length=1, color='blue')

    u, v, w = b_field(x, y, z)
    ax.quiver(x, y, z, u, v, w, length=1, color='red')


    print("ANIMATE DATA.",desc)
    ani=animation.FuncAnimation(fig, update_point, n_values, fargs=(values, points), blit=True)
    print("SAVE ANIMATION.",desc)
    writervideo = animation.FFMpegWriter(fps=30)
    ani.save(f_out, writer=writervideo)
    print("DONE.",desc)


if __name__ == '__main__':
    run(sys.argv[1],sys.argv[2],0)