pub fn _deserializable_sample_notebook() -> &'static str {
    r#"{
     "cells": [
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "Data Visualization with Matplotlib\n===",
        "\n",
        "\n",
        "This project is all about Matplotlib, the basic data visualization tool of Python programming language. I have discussed Matplotlib object hierarchy, various plot types with Matplotlib and customization techniques associated with Matplotlib. \n",
        "\n",
        "\n",
        "This project is divided into various sections based on contents which are listed below:- \n"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "Table of Contents\n===",
        "\n",
        "\n",
        "1.\tIntroduction\n",
        "\n",
        "2.\tOverview of Python Data Visualization Tools\n",
        "\n",
        "3.\tIntroduction to Matplotlib\n",
        "\n",
        "4.\tImport Matplotlib\n",
        "\n",
        "5.\tDisplaying Plots in Matplotlib\n",
        "\n",
        "6.\tMatplotlib Object Hierarchy\n",
        "\n",
        "7.\tMatplotlib interfaces\n",
        "\n",
        "8.\tPyplot API\n",
        "\n",
        "9.\tObject-Oriented API\n",
        "\n",
        "10.\tFigure and Subplots\n",
        "\n",
        "11.\tFirst plot with Matplotlib\n",
        "\n",
        "12.\tMultiline Plots\n",
        "\n",
        "13.\tParts of a Plot\n",
        "\n",
        "14.\tSaving the Plot\n",
        "\n",
        "15.\tLine Plot\n",
        "\n",
        "16.\tScatter Plot\n",
        "\n",
        "17.\tHistogram\n",
        "\n",
        "18.\tBar Chart\n",
        "\n",
        "19.\tHorizontal Bar Chart\n",
        "\n",
        "20.\tError Bar Chart\n",
        "\n",
        "21.\tMultiple Bar Chart\n",
        "\n",
        "22.\tStacked Bar Chart\n",
        "\n",
        "23.\tBack-to-back Bar Chart\n",
        "\n",
        "24.\tPie Chart\n",
        "\n",
        "25.\tBox Plot\n",
        "\n",
        "26.\tArea Chart\n",
        "\n",
        "27.\tContour Plot\n",
        "\n",
        "28.\tImage Plot\n",
        "\n",
        "29.\tPolar Chart\n",
        "\n",
        "30.\t3D Plotting with Matplotlib\n",
        "\n",
        "31.\tStyles with Matplotlib Plots\n",
        "\n",
        "32.\tAdding a grid\n",
        "\n",
        "33.\tHandling axes\n",
        "\n",
        "34.\tHandling X and Y ticks\n",
        "\n",
        "35.\tAdding labels\n",
        "\n",
        "36.\tAdding a title\n",
        "\n",
        "37.\tAdding a legend\n",
        "\n",
        "38.\tControl colours\n",
        "\n",
        "39.\tControl line styles\n",
        " \n",
        "40.\tSummary\n"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "1. Introduction\n",
        "\n",
        "\n",
        "When we want to convey some information to others, there are several ways to do so. The process of conveying the information with the help of plots and graphics is called **Data Visualization**. The plots and graphics take numerical data as input and display output in the form of charts, figures and tables. It helps to analyze and visualize the data clearly and make concrete decisions. It makes complex data more accessible and understandable. The goal of data visualization is to communicate information in a clear and efficient manner.\n",
        "\n",
        "\n",
        "In this project, I shed some light on **Matplotlib**, which is the basic data visualization tool of Python programming language. Python has different data visualization tools available which are suitable for different purposes. First of all, I will list these data visualization tools and then I will discuss Matplotlib.\n"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "2. Overview of Python Visualization Tools\n",
        "\n",
        "\n",
        "\n",
        "Python is the preferred language of choice for data scientists. Python have multiple options for data visualization. It has several tools which can help us to visualize the data more effectively. These Python data visualization tools are as follows:-\n",
        "\n",
        "\n",
        "\n",
        "•\tMatplotlib\n",
        "\n",
        "•\tSeaborn\n",
        "\n",
        "•\tpandas\n",
        "\n",
        "•\tBokeh\n",
        "\n",
        "•\tPlotly\n",
        "\n",
        "•\tggplot\n",
        "\n",
        "•\tpygal\n",
        "\n",
        "\n",
        "\n",
        "In the following sections, I discuss Matplotlib as the data visualization tool. \n"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "3. Introduction to Matplotlib\n",
        "\n",
        "\n",
        "**Matplotlib** is the basic plotting library of Python programming language. It is the most prominent tool among Python visualization packages. Matplotlib is highly efficient in performing wide range of tasks. It can produce publication quality figures in a variety of formats.  It can export visualizations to all of the common formats like PDF, SVG, JPG, PNG, BMP and GIF. It can create popular visualization types – line plot, scatter plot, histogram, bar chart, error charts, pie chart, box plot, and many more types of plot. Matplotlib also supports 3D plotting. Many Python libraries are built on top of Matplotlib. For example, pandas and Seaborn are built on Matplotlib. They allow to access Matplotlib's methods with less code. \n",
        "\n",
        "\n",
        "The project **Matplotlib** was started by John Hunter in 2002. Matplotlib was originally started to visualize Electrocorticography (ECoG) data of epilepsy patients during post-doctoral research in Neurobiology. The open-source tool Matplotlib emerged as the most widely used plotting library for the Python programming language. It was used for data visualization during landing of the Phoenix spacecraft in 2008.\n"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "\n",
        "4. Import Matplotlib\n",
        "\n",
        "Before, we need to actually start using Matplotlib, we need to import it. We can import Matplotlib as follows:-\n",
        "\n",
        "`import matplotlib`\n",
        "\n",
        "\n",
        "Most of the time, we have to work with **pyplot** interface of Matplotlib. So, I will import **pyplot** interface of Matplotlib as follows:-\n",
        "\n",
        "\n",
        "`import matplotlib.pyplot`\n",
        "\n",
        "\n",
        "To make things even simpler, we will use standard shorthand for Matplotlib imports as follows:-\n",
        "\n",
        "\n",
        "`import matplotlib.pyplot as plt`\n",
        "\n"
       ]
      },
      {
       "cell_type": "code",
       "execution_count": 1,
       "metadata": {},
       "outputs": [],
       "source": [
        "Import dependencies\n",
        "\n",
        "import numpy as np\n",
        "import pandas as pd"
       ]
      },
      {
       "cell_type": "code",
       "execution_count": 2,
       "metadata": {},
       "outputs": [],
       "source": [
        "Import Matplotlib\n",
        "\n",
        "import matplotlib.pyplot as plt "
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "5. Displaying Plots in Matplotlib\n",
        "\n",
        "\n",
        "Viewing the Matplotlib plot is context based. The best usage of Matplotlib differs depending on how we are using it. \n",
        "There are three applicable contexts for viewing the plots. The three applicable contexts are using plotting from a script, plotting from an IPython shell or plotting from a Jupyter notebook.\n"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "Plotting from a script\n",
        "\n",
        "\n",
        "\n",
        "If we are using Matplotlib from within a script, then the **plt.show()** command is of great use. It starts an event loop, \n",
        "looks for all currently active figure objects, and opens one or more interactive windows that display the figure or figures.\n",
        "\n",
        "\n",
        "The **plt.show()** command should be used only once per Python session. It should be used only at the end of the script. Multiple **plt.show()** commands can lead to unpredictable results and should mostly be avoided.\n"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "Plotting from an IPython shell\n",
        "\n",
        "\n",
        "We can use Matplotlib interactively within an IPython shell. IPython works well with Matplotlib if we specify Matplotlib mode. To enable this mode, we can use the **%matplotlib** magic command after starting ipython. Any plt plot command will cause a figure window to open and further commands can be run to update the plot.\n",
        "\n"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "Plotting from a Jupyter notebook\n",
        "\n",
        "\n",
        "The Jupyter Notebook (formerly known as the IPython Notebook) is a data analysis and visualization tool that provides multiple tools under one roof.  It provides code execution, graphical plots, rich text and media display, mathematics formula and much more facilities into a single executable document.\n",
        "\n",
        "\n",
        "Interactive plotting within a Jupyter Notebook can be done with the **%matplotlib** command. There are two possible options to work with graphics in Jupyter Notebook. These are as follows:-\n",
        "\n",
        "\n",
        "•\t**%matplotlib notebook** – This command will produce interactive plots embedded within the notebook.\n",
        "\n",
        "•\t**%matplotlib inline** – It will output static images of the plot embedded in the notebook.\n",
        "\n",
        "\n",
        "After this command (it needs to be done only once per kernel per session), any cell within the notebook that creates a plot will embed a PNG image of the graphic.\n"
       ]
      },
      {
       "cell_type": "code",
       "execution_count": 3,
       "metadata": {},
       "outputs": [
        {
         "data": {
          "image/png": "i=\n",
          "text/plain": [
           "<Figure size 432x288 with 1 Axes>"
          ]
         },
         "metadata": {
          "needs_background": "light"
         },
         "output_type": "display_data"
        }
       ],
       "source": [
        "%matplotlib inline\n",
        "\n",
        "\n",
        "x1 = np.linspace(0, 10, 100)\n",
        "\n",
        "\n",
        "create a plot figure\n",
        "fig = plt.figure()\n",
        "\n",
        "plt.plot(x1, np.sin(x1), '-')\n",
        "plt.plot(x1, np.cos(x1), '--');"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "6. Matplotlib Object Hierarchy\n",
        "\n",
        "\n",
        "There is an Object Hierarchy within Matplotlib. In Matplotlib, a plot is a hierarchy of nested Python objects. \n",
        "A**hierarch** means that there is a tree-like structure of Matplotlib objects underlying each plot.\n",
        "\n",
        "\n",
        "A **Figure** object is the outermost container for a Matplotlib plot. The **Figure** object contain multiple **Axes** objects. So, the **Figure** is the final graphic that may contain one or more **Axes**. The **Axes** represent an individual plot.\n",
        "\n",
        "\n",
        "So, we can think of the **Figure** object as a box-like container containing one or more **Axes**. The **Axes** object contain smaller objects such as tick marks, lines, legends, title and text-boxes.\n"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "7.\tMatplotlib API Overview\n",
        "\n",
        "\n",
        "\n",
        "Matplotlib has two APIs to work with. A MATLAB-style state-based interface and a more powerful object-oriented (OO) interface. \n",
        "The former MATLAB-style state-based interface is called **pyplot interface** and the latter is called **Object-Oriented** interface.\n",
        "\n",
        "\n",
        "There is a third interface also called **pylab** interface. It merges pyplot (for plotting) and NumPy (for mathematical functions) together in an environment closer to MATLAB. This is considered bad practice nowadays. So, the use of **pylab** is strongly discouraged and hence, I will not discuss it any further.\n"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "8. Pyplot API \n",
        "\n",
        "\n",
        "**Matplotlib.pyplot** provides a MATLAB-style, procedural, state-machine interface to the underlying object-oriented library in Matplotlib. **Pyplot** is a collection of command style functions that make Matplotlib work like MATLAB. Each pyplot function makes some change to a figure - e.g., creates a figure, creates a plotting area in a figure etc. \n",
        "\n",
        "\n",
        "**Matplotlib.pyplot** is stateful because the underlying engine keeps track of the current figure and plotting area information and plotting functions change that information. To make it clearer, we did not use any object references during our plotting we just issued a pyplot command, and the changes appeared in the figure.\n",
        "\n",
        "\n",
        "We can get a reference to the current figure and axes using the following commands-\n",
        "\n",
        "\n",
        "`plt.gcf ( )`   # get current figure\n",
        "\n",
        "`plt.gca ( )`   # get current axes \n",
        "\n",
        " \n",
        "**Matplotlib.pyplot** is a collection of commands and functions that make Matplotlib behave like MATLAB (for plotting). \n",
        "The MATLAB-style tools are contained in the pyplot (plt) interface. \n",
        "\n",
        "This is really helpful for interactive plotting, because we can issue a command and see the result immediately. But, it is not suitable for more complicated cases. For these cases, we have another interface called **Object-Oriented** interface, described later.\n"
       ]
      },
      {
       "cell_type": "markdown",
       "metadata": {},
       "source": [
        "The following code produces sine and cosine curves using Pyplot API."
       ]
      },
      {
       "cell_type": "code",
       "execution_count": 4,
       "metadata": {},
       "outputs": [
        {
         "data": {
          "image/png": "ig==\n",
          "text/plain": [
           "<Figure size 432x288 with 2 Axes>"
          ]
         },
         "metadata": {
          "needs_background": "light"
         },
         "output_type": "display_data"
        }
       ],
       "source": [
        "create a plot figure\n",
        "plt.figure()\n",
        "\n",
        "\n",
        "create the first of two panels and set current axis\n",
        "plt.subplot(2, 1, 1)   # (rows, columns, panel number)\n",
        "plt.plot(x1, np.sin(x1))\n",
        "\n",
        "\n",
        "create the second of two panels and set current axis\n",
        "plt.subplot(2, 1, 2)   # (rows, columns, panel number)\n",
        "plt.plot(x1, np.cos(x1));\n"
       ]
      },
      {
       "cell_type": "code",
       "execution_count": 5,
       "metadata": {},
       "outputs": [
        {
         "name": "stdout",
         "output_type": "stream",
         "text": [
          "Figure(432x288)\n"
         ]
        },
        {
         "data": {
          "text/plain": [
           "<Figure size 432x288 with 0 Axes>"
          ]
         },
         "metadata": {},
         "output_type": "display_data"
        }
       ],
       "source": [
        "get current figure information\n",
        "\n",
        "print(plt.gcf())"
       ]
      },
      {
       "cell_type": "code",
       "execution_count": 6,
       "metadata": {},
       "outputs": [
        {
         "name": "stdout",
         "output_type": "stream",
         "text": [
          "AxesSubplot(0.125,0.125;0.775x0.755)\n"
         ]
        },
        {
         "data": {
          "image/png": "=\n",
          "text/plain": [
           "<Figure size 432x288 with 1 Axes>"
          ]
         },
         "metadata": {
          "needs_background": "light"
         },
         "output_type": "display_data"
        }
       ],
       "source": [
        "get current axis information\n",
        "\n",
        "print(plt.gca())"
       ]
      }
     ]
    }"#
}
