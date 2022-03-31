import styled from '@emotion/styled';
import { ChangeEvent, useCallback, useEffect, useState, VFC } from 'react';
import { useRecoilState, useRecoilValue, useSetRecoilState } from 'recoil';
import { atom_channels, atom_tasks } from '../../stores/tasks';

const Container = styled.div`
    display: grid;
    grid-template-columns: 20vw 80vw;
    background-color: aquamarine;
    min-height: 100vh;
`;

const SidebarWrapper = styled.div`
    background-color: #f5f5f5;
`;

const MainContentWrapper = styled.div`
    margin: 10px;
    padding: 10px;
    background-color: aquamarine;
    display: grid;
    grid-template-rows: 60px 1fr;

    & > .task-view {
        /* background-color: aquamarine; */
        height: 100%;
        overflow: scroll;
    }
`;

const ChannelList = styled.div`
    padding: 10px;
    background-color: aquamarine;
    &:hover {
        background-color: #7070f1;
    }
`;

const Sidebar: VFC = () => {
    const [channels, setChannels] = useRecoilState(atom_channels);
    const selectChannel = useCallback(
        (name: string) => {
            setChannels({ ...channels, select: name });
        },
        [channels.channels],
    );
    return (
        <SidebarWrapper>
            <div style={{ padding: '10px' }}>Sidebar</div>
            {channels.channels.map((channel) => {
                let onclick = () => {
                    selectChannel(channel.name);
                };
                return (
                    <ChannelList key={channel.name} onClick={onclick}>
                        # {channel.name}
                    </ChannelList>
                );
            })}
        </SidebarWrapper>
    );
};

const ListWrapper = styled.div`
    display: flex;
    margin: 10px;
`;

const ListComponent = styled.div`
    flex-shrink: 0;
    padding: 10px;
    margin: 10px;
    width: 25%;
    background-color: #f5f5f5;
    border-radius: 5px;
`;

const ModalWindow = styled.div`
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    padding: 10vh 10vw;
    background: rgba(21, 21, 21, 0.4);
`;

const ModalContent = styled.div`
    width: 80vw;
    height: 80vh;
    padding: 20px;
    border-radius: 10px;
    background-color: #f5f5f5;
`;

enum ModalContentType {
    AddTask,
    EditTask,
}

interface TaskAddComponentProps {
    list: string;
    channel: string;
}

const TaskAddComponent: VFC<TaskAddComponentProps> = ({ list, channel: ch }) => {
    const [inputTask, setInputTask] = useState({ name: '', list: list });
    const channels = useRecoilValue(atom_channels);
    const [tasks, setTasks] = useRecoilState(atom_tasks);
    
    const onclick = () => {
        alert(JSON.stringify(inputTask));
        setTasks({ ...tasks, tasks: [...tasks.tasks, inputTask] });
    };
    const setText = (e: ChangeEvent<HTMLInputElement>) => {
        setInputTask({ ...inputTask, name: e.target.value });
    };
    const setList = (e: ChangeEvent<HTMLSelectElement>) => {
        setInputTask({ ...inputTask, list: e.target.value });
    };
    return (
        <div>
            <div>Add Task</div>
            <div>
                <input type="text" placeholder="task name" onChange={setText} />
            </div>
            <div>
                <input type="datetime-local" name="" id="" />
            </div>
            <select name="task-list" id="task-list" value={inputTask.list} onChange={setList}>
                {channels.channels
                    .filter((channel) => channel.name === ch)[0]
                    ?.lists.map((list) => {
                        return <option key={list}>{list}</option>;
                    })}
            </select>
            <button onClick={onclick}>Add</button>
        </div>
    );
};

const MainContent: VFC = () => {
    const [channels, setChannels] = useRecoilState(atom_channels);

    const tasks = useRecoilValue(atom_tasks);
    const [toggle, setToggle] = useState(false);
    const [select_list, setSelectList] = useState('');
    const [modalContentType, setModalContentType] = useState<ModalContentType>(ModalContentType.AddTask);
    const modalOpen = () => {
        setModalContentType(ModalContentType.EditTask);
        setToggle(true);
    };

    const modalTaskAddOpen = (list_name: string) => {
        setModalContentType(ModalContentType.AddTask);
        setSelectList(list_name);
        setToggle(true);
    };

    useEffect(() => {
        if (!channels.select && channels.channels.length > 0) {
            setChannels({ ...channels, select: channels.channels[0].name });
        }
    }, []);

    return (
        <MainContentWrapper>
            <div style={{ padding: '10px' }}># {channels.select}</div>
            {toggle && (
                <ModalWindow
                    onClick={() => {
                        setToggle(false);
                    }}
                >
                    <ModalContent
                        onClick={(e) => {
                            e.stopPropagation();
                        }}
                    >
                        modal content
                        {modalContentType === ModalContentType.AddTask && (
                            <TaskAddComponent list={select_list} channel={channels.select} />
                        )}
                        {modalContentType === ModalContentType.EditTask && <div>edit task</div>}
                    </ModalContent>
                </ModalWindow>
            )}
            <div className="task-view">
                <ListWrapper>
                    {channels.channels
                        .find((ch) => ch.name == channels.select)
                        ?.lists.map((list) => {
                            return (
                                <ListComponent>
                                    <div>
                                        <b>{list}</b>
                                    </div>
                                    <div>
                                        {tasks.tasks
                                            .filter((task) => task.list == list)
                                            .map((task) => {
                                                return <div onClick={modalOpen}>{task.name}</div>;
                                            })}
                                    </div>
                                    <AddTaskDiv onClick={() => modalTaskAddOpen(list)}>
                                        <div>Task を追加</div>
                                        <div>+</div>
                                    </AddTaskDiv>
                                </ListComponent>
                            );
                        })}
                </ListWrapper>
            </div>
        </MainContentWrapper>
    );
};

const AddTaskDiv = styled.div`
    margin-top: 10px;
    padding: 5px;
    border-top: 1px solid #151515;
    display: flex;
    justify-content: space-between;
`;

const TaskPage: VFC = () => {
    return (
        <Container>
            <Sidebar />
            <MainContent />
        </Container>
    );
};

export default TaskPage;
