import styled from '@emotion/styled';
import { channel } from 'diagnostics_channel';
import { ChangeEvent, useCallback, useEffect, useState, VFC } from 'react';
import { useRecoilState, useRecoilValue } from 'recoil';
import { atom_channels } from '../../stores/tasks';

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

interface Tasks {
    lists: string[];
    tasks: Task[];
}

interface Task {
    name: string;
    list: string;
}

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
    list: String;
}

const TaskAddComponent: VFC<TaskAddComponentProps> = ({ list }) => {
    const [inputTask, setInputTask] = useState({ name: '', list: list });

    const onclick = () => {
        alert(JSON.stringify(inputTask));
    };
    const setText = (e: ChangeEvent<HTMLInputElement>) => {
        setInputTask({ ...inputTask, name: e.target.value });
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
            <button onClick={onclick}>Add</button>
        </div>
    );
};

const MainContent: VFC = () => {
    const [channels, setChannels] = useRecoilState(atom_channels);
    const tasks: Tasks = {
        lists: ['list1', 'list2', 'list3', 'list4', 'list5'],
        tasks: [
            { name: 'task1', list: 'list1' },
            { name: 'task2', list: 'list1' },
            { name: 'task3', list: 'list2' },
            { name: 'task4', list: 'list2' },
        ],
    };
    const [toggle, setToggle] = useState(false);
    const [modalContentType, setModalContentType] = useState<ModalContentType>(ModalContentType.AddTask);
    const modalOpen = () => {
        setModalContentType(ModalContentType.EditTask);
        setToggle(true);
    };

    const modalTaskAddOpen = () => {
        setModalContentType(ModalContentType.AddTask);
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
                        {modalContentType === ModalContentType.AddTask && <TaskAddComponent list="list" />}
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
                                    <AddTaskDiv onClick={modalTaskAddOpen}>
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
