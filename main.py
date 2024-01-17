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
            human._World = self
            human._WorldIndex = i
            human._Position = self._HumanPositions[i]
            human._Satisfaction = self._HumanSatisfactions[i]
            i += 1
        self._Forests = forests
        self._Lakes = lakes
        
    def UpdateWorld(self):
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
    
    def UpdateDraw(self, world, fig, ax, background, humanPoints):
            # restore background
            fig.canvas.restore_region(background)

            for human in world._Humans :
                humanPoints[human._WorldIndex].set_data(human._Position[0] , human._Position[1])
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
    _World = None
    _WorldIndex = -1
    _CurrentAction = None
    _CachedDestination = np.array([0, 0])
    _CachedDuration = 0
    _Commuting = False
    _TimeSpentOnAction = 0

    def __init__(self, position, satisfaction = DEFAULT_SATISFACTION):
        self._Position = position
        self._Satisfaction = satisfaction

    def UpdateTime(self):
        for needIndex in range(len(HUMAN_NEEDS)):
            self._Satisfaction[needIndex] = HUMAN_NEEDS[needIndex]._Depletion(self._Satisfaction[needIndex])
        if self._CurrentAction != None:
            self._CurrentAction.Update(self, self._World)

    def Reward(self):
        reward = 0
        for need, satisfaction in zip(HUMAN_NEEDS, self._Satisfaction):
            reward += need.Cost(satisfaction)
        return reward
    
    def PredictNextAction():
        return 

    def DoAction(self, action, world):
        if self._CurrentAction != None:
            return
        self._CurrentAction = action
        action.Do(self, world)


class Need:
    _Reward = lambda x : 0
    _Depletion = lambda x : x

    def __init__ (self, reward, depletion):
        self._Reward = reward
        self._Depletion = depletion


class Action:
    _Needs = []
    _Satisfaction = lambda h, w : []
    _Duration = lambda h, w : 1
    _Destination = lambda h, w : h._Position

    def __init__(self, needs = [], satisfaction = None, duration = None, destination = None):
        self._Needs = needs
        if satisfaction is not None:
            self._Satisfaction = satisfaction
        if duration is not None:
            self._Duration = duration
        if destination is not None:
            self._Destination = destination

    def Do(self, human, world):
        if self._Destination is not None:
            human._CachedDestination = self._Destination(human, world)
        human._Commuting = Norm(human._CachedDestination - human._Position) > 0.01
        human._CachedDuration = self._Duration(human, world)
        human._TimeSpentOnAction = 0

    def Update(self, human, world):
        if human._Commuting:
            human._Position[0], human._Position[1] = human._Position + (human._CachedDestination - human._Position) / Norm(human._CachedDestination - human._Position) * WALKING_VELOCITY
            human._Commuting = Norm(human._CachedDestination - human._Position) > 0.01
            return
        
        if (human._TimeSpentOnAction < human._CachedDuration):
            human._TimeSpentOnAction += 1
            return
        
        out_satisfaction = self._Satisfaction(human._Satisfaction, world)
        index = 0
        for need in self._Needs:
            human._Satisfaction[HUMAN_NEEDS_INDICES[need]] = out_satisfaction[index]

    def Predict(self, human):
        if self._Needs:
            return self._Reward(human._Satisfaction[HUMAN_NEEDS_INDICES[self._Needs]])
        
    def TimeNeeded(self, human, world):
        return self._Duration(human, world)

def Norm(v):
    return np.linalg.norm(v)

def FindNearest(pos, element_list):
    if (len(element_list) == 0):
        return None
    elif (len(element_list) == 1):
        return element_list[0]
    nearest_element = element_list[0]
    min_dist = max(0, Norm(pos - element_list[0][0:2]) - element_list[0][2])
    for element in element_list[1:]: 
        dist = max(0, Norm(pos - element[0:2]) - element[2])
        if dist < min_dist:
            min_dist = dist
            nearest_element = element
    return nearest_element

def FindCircleEdge(pos, destination):
    dest_pos = destination[:2]
    dist = Norm(dest_pos - pos)
    if (dist < destination[2]):
        return pos
    direction = (dest_pos - pos) / dist
    return dest_pos - direction * destination[2]

def FindClosestLake(human, world):
    pos = human._Position
    destination = FindNearest(pos,  world._Lakes)
    return FindCircleEdge(pos, destination)

def FindSquareEdge(pos, vel, element_list):
    destination = FindNearest(pos, element_list)
    dest_pos = destination[:2] + destination[3:] / 2
    vec = dest_pos - pos
    direction = (vec) / Norm(vec)
    if (abs(vec[0]) > destination[2] / 2 or abs(vec[1]) > destination[3] /2):
        return pos + (vel * direction)
    return pos

# Make Needs
nFood = Need(reward = lambda x : (50-x) * 2 if x >= 0 else DEATH_VALUE, depletion =  lambda x : x - 1)
nWater = Need(reward = lambda x : (50-x) * 4 if x >= 0 else DEATH_VALUE, depletion =  lambda x : x - 2)
nRest = Need(reward = lambda x : (50-x), depletion =  lambda x : x - 1)
nPoo = Need(reward = lambda x : (50-x) / 2, depletion =  lambda x : x - 1)
HUMAN_NEEDS = [nFood, nWater, nRest, nPoo]
HUMAN_NEEDS_INDICES = dict()
for index in range(len(HUMAN_NEEDS)):
    HUMAN_NEEDS_INDICES[HUMAN_NEEDS[index]] = index

DEFAULT_SATISFACTION = [100, 100, 100, 100]
WALKING_VELOCITY = 1.0

# Make Actions 
aMove = Action(needs = [], satisfaction = None, duration = None, destination = lambda h, w : [h._Position[0] + WALKING_VELOCITY, h._Position[1]])
aEat = Action(needs = [nFood], satisfaction = lambda h, w : [100], duration = lambda h, w : 3)
aDrink = Action(needs = [nWater], satisfaction = lambda h, w : [100], duration = lambda h, w : 1, destination = FindClosestLake)
aSleep = Action(needs = [nFood, nWater, nRest, nPoo], satisfaction = lambda h, w : [h._Satisfaction[0], h._Satisfaction[1], 100, h._Satisfaction[2]], duration = lambda h, w : 30)
aPoop = Action(needs = [nPoo], satisfaction = lambda h, w : [100], duration = lambda h, w : 2)

POSSIBLE_ACTIONS = [aEat, aDrink, aSleep, aPoop]

characters = [Human( [30.0, 20.0],DEFAULT_SATISFACTION), Human([50.0, 40.0], DEFAULT_SATISFACTION)]

# Define the positions of characters, forests, and lakes
forests = np.array([[10.0, 10.0, 30.0, 40.0], [70.0, 40.0, 20.0, 20.0]])
lakes = np.array([[40.0, 80.0, 5.0], [60.0, 30.0, 10.0]])

myWorld = World(100, characters, forests, lakes)

#Initial Draw
fig, ax = myWorld.InitDraw()
background = myWorld.DrawBackground(fig, ax)
humanPoints = myWorld.DrawHumans(ax)

plt.ion()
for human in myWorld._Humans : 
    human.DoAction(aDrink, myWorld)
for i in range(100):
    myWorld.UpdateWorld()
    myWorld.UpdateDraw(myWorld, fig, ax, background, humanPoints)
    plt.pause(0.033)  # Add a short pause to observe the updates

plt.ioff()
plt.show()