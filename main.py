import matplotlib.pyplot as plt
import numpy as np
import time

class World :
    # Define the size of the world
    _Size = 100
    _CurentTime = 0
    _Humans = []
    _Forests = np.array([])
    _Lakes = np.array([])
    _HumanPositions = np.array([])
    _HumanSatisfactions = np.array([])

    def __init__(self, world_size, humans, forests, lakes):
        self._Size =  world_size 
        self._Humans = humans
        self._HumanPositions = np.array([human._Position for human in humans])# should be shared
        self._HumanSatisfactions = np.array([human._Satisfaction for human in humans])
        i = 0
        for human in humans:
            human._WorldIndex = i
            human._Position = self._HumanPositions[i]
            human._Satisfaction = self._HumanSatisfactions[i]
            i += 1
        self._Forests = forests
        self._Lakes = lakes
        
    def UpdateTime(self):
        for human in self._Humans:
            human.UpdateTime()

    def InitDraw(self):
        
        # Create a figure and axis
        fig, ax = plt.subplots()

        # Plot the world boundary
        ax.set_xlim(0, self._Size)
        ax.set_ylim(0, self._Size)


        # Add labels
        ax.set_title('MDP Simulated Society')
        ax.legend()
        return fig, ax

    def DrawBackground(self, fig, ax):
        # Plot forests as rectangles
        for forest in self._Forests:
            rect = plt.Rectangle((forest[0], forest[1]), forest[2], forest[3], color='green', alpha=0.5)
            ax.add_patch(rect)

        # Plot lakes as circles
        for lake in self._Lakes:
            circle = plt.Circle((lake[0], lake[1]), lake[2], color='blue', alpha=0.5)
            ax.add_patch(circle)
            
        background = fig.canvas.copy_from_bbox(ax.bbox)
        return background

    def DrawHumans(self, ax):
        humanPoints = []
        for humanPos in self._HumanPositions:
            # Plot characters as points
            humanPoints.append( ax.plot(humanPos[0] , humanPos[1], marker='o', color='red', label='Characters')[0])
        return humanPoints
    
    def UpdateDraw(self, fig, ax, background, humanPoints):
            # restore background
            fig.canvas.restore_region(background)

            # redraw just the points
            for point in humanPoints:
                ax.draw_artist(point)

            # fill in the axes rectangle
            fig.canvas.blit(ax.bbox)



HUMAN_NEEDS = []
DEFAULT_SATISFACTION = []
POSSIBLE_ACTIONS = []
DEATH_VALUE = -1e10

class Human:
    _WorldIndex = -1

    def __init__(self, position, satisfaction = DEFAULT_SATISFACTION):
        self._Position = position
        self._Satisfaction = satisfaction

    def UpdateTime(self):
        for need in HUMAN_NEEDS:
            self._Satisfaction[need] = need._UpdateTime(self._Satisfaction[need])

    def Reward(self):
        reward = 0
        for need, satisfaction in zip(HUMAN_NEEDS, self._Satisfaction):
            reward += need.Cost(satisfaction)
        return reward
    
    def PredictNextAction():
        return 

    def DoAction(self, action):
        action.Do(self)


class Need:
    _Cost = lambda x : 0
    _UpdateTime = lambda x : x

    def __init__ (self, cost, func):
        self._Cost = cost
        self._UpdateCost = func


class Action:
    _Need = None
    _Result = lambda x : x
    _TimeNeed = lambda t : 1
    _EndPosition = lambda pos : pos

    def __init__(self, need = None, result = None, timeNeeded = None, endPosition = None):
        self._Need = need
        if result is not None:
            self._Result = result
        if timeNeeded is not None:
            self._TimeNeed = timeNeeded
        if endPosition is not None:
            self._EndPosition = endPosition

    def Do(self, human):
        if self._Need:
            human._Satisfaction[self._Need][0], human._Satisfaction[self._Need][1] = self._Result(human._Satisfaction[self._Need])
        if self._EndPosition is not None:
            human._Position[0], human._Position[1] = self._EndPosition(human._Position)
            human._PositionDirty = True

    def Predict(self, human):
        if self._Need:
            return self._Result(human._Satisfaction[self._Need])
        
    def TimeNeeded(self, human, world):
        return self._TimeNeed(world._CurentTime)

# Make Needs
nFood = Need(lambda x : (50-x) * 2 if x >= 0 else DEATH_VALUE, lambda x : x - 1)
nWater = Need(lambda x : (50-x) * 4 if x >= 0 else DEATH_VALUE, lambda x : x - 2)
nRest = Need(lambda x : (50-x), lambda x : x - 1)
nPoo = Need(lambda x : (50-x) / 2, lambda x : x - 1)
HUMAN_NEEDS = [nFood, nWater, nRest, nPoo]
DEFAULT_SATISFACTION = [100, 100, 100, 100]


# Make Actions 
aMove = Action(None, None, None, lambda x : [x[0]+1, x[1]])
aEat = Action(nFood, lambda x : 100, lambda t : 3)
aDrink = Action(nWater, lambda x : 100, lambda t : 3)
aSleep = Action(nRest, lambda x : 100, lambda t : 3)
aPoop = Action(nPoo, lambda x : 100, lambda t : 3)

POSSIBLE_ACTIONS = [aEat, aDrink, aSleep, aPoop]

characters = [Human( [30, 20],DEFAULT_SATISFACTION), Human([50, 40], DEFAULT_SATISFACTION)]

# Define the positions of characters, forests, and lakes
forests = np.array([[10, 10, 30, 40], [70, 40, 20, 20]])
lakes = np.array([[40, 80, 5], [60, 30, 10]])

myWorld = World(100, characters, forests, lakes)

print(myWorld._HumanPositions)
print(myWorld._HumanSatisfactions)


#Initial Draw
fig, ax = myWorld.InitDraw()
background = myWorld.DrawBackground(fig, ax)
humanPoints = myWorld.DrawHumans(ax)
print(humanPoints)
plt.ion()
for i in range(100):
    for human in myWorld._Humans : 
        human.DoAction(aMove)
        humanPoints[human._WorldIndex].set_data(human._Position[0] , human._Position[1])
    myWorld.UpdateDraw(fig, ax, background, humanPoints)
    plt.pause(0.033)  # Add a short pause to observe the updates

plt.ioff()
plt.show()