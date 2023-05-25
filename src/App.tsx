import { useState, useCallback } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
  Connection,
} from 'reactflow';

import 'reactflow/dist/style.css';

import { Button } from 'semantic-ui-react';

import { usePipeline } from './pipeline';

import {
  Edge,
  InputPort,
  OutputPort,
  Position,
  createComponent,
  setAssignment,
  addEdge,
  removeEdge,
} from './bindings';
import { ContextMenu } from './ContextMenu';
import { AssignmentModal } from './AssignmentModal';

export type ContextMenuState = {
  open: boolean;
  x: number;
  y: number;
};

type AssignModalState = {
  open: boolean;
  source?: OutputPort;
  target?: InputPort;
  edge?: Edge;
};

function App() {
  const [modal, setModal] = useState<AssignModalState>({ open: false });
  const { onApply, ...pipeline } = usePipeline({
    onAssignEdit: (edge: Edge, source: OutputPort, target: InputPort) => {
      setModal({ open: true, edge, source, target });
    },
    onAddEdge: (connection: Connection) => {
      addEdge(
        connection.source ?? '',
        connection.target ?? '',
        connection.sourceHandle ?? '',
        connection.targetHandle ?? '',
      );
    },
    onRemoveEdge: (connection: Connection) => {
      removeEdge(
        connection.source ?? '',
        connection.target ?? '',
        connection.sourceHandle ?? '',
        connection.targetHandle ?? '',
      );
    },
  });

  const onAssign = useCallback(
    (id: string, assignment: Record<string, string>) => {
      setAssignment(id, assignment);
      setModal({ open: false });
    },
    [],
  );

  const [menu, setMenu] = useState<ContextMenuState>({
    open: false,
    x: 0,
    y: 0,
  });
  const onPaneContextMenu = useCallback(
    (e: React.MouseEvent) => {
      setMenu({ open: true, x: e.clientX, y: e.clientY });
    },
    [setMenu],
  );
  const onMenuClose = useCallback(() => {
    setMenu({ open: false, x: 0, y: 0 });
  }, [setMenu]);
  const onMenuClick = useCallback(async (type: string, pos: Position) => {
    await createComponent(type, pos);
  }, []);

  return (
    <div className="container">
      <ReactFlow
        {...pipeline}
        fitView={true}
        onPaneContextMenu={onPaneContextMenu}
      >
        <MiniMap />
        <Controls />
        <Background />
      </ReactFlow>
      <Button onClick={onApply} primary>
        Apply
      </Button>
      <ContextMenu
        {...menu}
        onMenuClose={onMenuClose}
        onMenuClick={onMenuClick}
      />
      <AssignmentModal
        {...modal}
        onAssign={onAssign}
        onDiscard={() => setModal({ open: false })}
      />
    </div>
  );
}

export default App;
