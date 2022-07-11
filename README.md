# Shenzhen I/O Solitaire Solver

## Python Setup

```
python -m venv .venv
. .venv/bin/activate
python -m pip install -r requirements.txt
```

This should set up everything. Jupyter lab might not display interactive
matplotlib widgets with an error similar to this issue:
https://github.com/matplotlib/ipympl/issues/456

In that case, try this command to see if there is a conflicting version of the
extension installed system wide:

```
jupyter labextension list
```

Remove any system wide extensions and rerun the command to verify that only
extensions from the venv are used:

```
$ jupyter labextension list
JupyterLab v3.4.3
.venv/share/jupyter/labextensions
        jupyterlab_pygments v0.2.2 enabled OK (python, jupyterlab_pygments)
        jupyter-matplotlib v0.11.1 enabled OK
        jupyterlab-jupytext v1.3.8+dev enabled OK (python, jupytext)
        @jupyter-widgets/jupyterlab-manager v3.1.1 enabled OK (python, jupyterlab_widgets)
```
