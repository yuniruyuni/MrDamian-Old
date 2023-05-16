import { useCallback, useMemo, useEffect, useRef } from "react";
import type { Node as RFNode, Edge as RFEdge, Connection, HandleType } from 'reactflow';
import {
  useNodesState,
  useEdgesState,
  addEdge,
  updateEdge
} from 'reactflow';

import { PropertiesNode } from "./PropertiesNode";
import { updatePipeline, pipeline, Node, Edge, InputPort, OutputPort } from './bindings';

type NodeWithEvent = Node & {
  data: {
    onInputAssignEdit: (i: InputPort) => void,
    onOutputAssignEdit: (o: OutputPort) => void,
  },
};

export function usePipeline(
  onInputAssignEdit: (node: Node, i: InputPort) => void,
  onOutputAssignEdit: (node: Node, o: OutputPort) => void,
) {
  const edgeUpdateSuccessful = useRef(true);

  const nodeTypes = useMemo(() => ({
    TwitchSubscriber: PropertiesNode,
    TwitchPublisher: PropertiesNode,
  }), []);
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  const onEdgeUpdateStart = useCallback(() => {
    edgeUpdateSuccessful.current = false;
  }, []);
  const onEdgeUpdate = useCallback((oldEdge: RFEdge, newConnection: Connection) => {
    edgeUpdateSuccessful.current = true;
    setEdges((edges) => updateEdge(oldEdge, newConnection, edges));
  }, []);
  const onEdgeUpdateEnd = useCallback((_event: MouseEvent | TouchEvent, target: RFEdge, _handle: HandleType) => {
    if (!edgeUpdateSuccessful.current) {
      setEdges((edges) => edges.filter((e) => e.id !== target.id));
    }
    edgeUpdateSuccessful.current = true;
  }, []);

  const onConnect = useCallback(
    (connection: Connection) => setEdges((edges) => addEdge(connection, edges)),
    [setEdges],
  );

  const onApply = useCallback(() => {
    (async () => {
      const rnodes: Node[] = nodes
        .map((node) => ({ ...node, type: node.type??'' }));
      const redges: Edge[] = edges.map((edge) => ({
        ...edge,
        label: edge.label?.toString()??'',
        sourceHandle: edge.sourceHandle??'',
        targetHandle: edge.targetHandle??'',
      }));

      await updatePipeline({ nodes: rnodes, edges: redges });
    })()
  }, [nodes, edges]);

  const extendNode = (node: Node) => ({
    ...node,
    data: {
      ...node.data,
      onInputAssignEdit: (i: InputPort) => { onInputAssignEdit(node, i); },
      onOutputAssignEdit: (o: OutputPort) => { onOutputAssignEdit(node, o); },
    }
  });

  const addNode = useCallback((node: Node) => {
    setNodes((nodes) => [...nodes, extendNode(node)]);
  }, [setNodes]);

  useEffect(() => {
    (async () => {
      const { nodes, edges } = await pipeline();
      const nodesWithEvents = nodes.map((node) => extendNode(node));
      setNodes(nodesWithEvents);
      setEdges(edges);
    })()
  }, []);

  return {
    nodes,
    edges,
    nodeTypes,
    onNodesChange,
    onEdgesChange,
    onEdgeUpdateStart,
    onEdgeUpdate,
    onEdgeUpdateEnd,
    onConnect,
    onApply,
    addNode,
  };
}
